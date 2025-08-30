use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::Mutex;
use uuid::Uuid;

// Define the EmbeddingRecord struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingRecord {
    pub id: String,
    pub name: String,
    pub embedding: Vec<f32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub metadata: HashMap<String, String>,
}

// Define the EmbeddingStorage trait
pub trait EmbeddingStorage {
    fn store_embedding(&mut self, record: EmbeddingRecord) -> Result<()>;
    fn get_embedding(&self, id: &str) -> Result<Option<EmbeddingRecord>>;
    fn get_all_embeddings(&self) -> Result<Vec<EmbeddingRecord>>;
    fn delete_embedding(&mut self, id: &str) -> Result<bool>;
}

// Simple local file storage implementation
pub struct LocalFileStorage {
    file_path: String,
    data: Mutex<HashMap<String, EmbeddingRecord>>,
}

impl LocalFileStorage {
    pub fn new(file_path: String) -> Result<Self> {
        let storage = LocalFileStorage {
            file_path,
            data: Mutex::new(HashMap::new()),
        };
        
        // Load existing data if file exists
        storage.load_data()?;
        Ok(storage)
    }

    fn load_data(&self) -> Result<()> {
        let path = Path::new(&self.file_path);
        if !path.exists() {
            return Ok(());
        }

        // Check if file is empty
        let metadata = fs::metadata(path)?;
        if metadata.len() == 0 {
            // File exists but is empty, this is fine - just use empty HashMap
            return Ok(());
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        // Try to parse JSON, if it fails due to empty/invalid content, start fresh
        let data: HashMap<String, EmbeddingRecord> = match serde_json::from_reader(reader) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Warning: Could not parse existing embeddings file ({e}), starting fresh");
                HashMap::new()
            }
        };
        
        if let Ok(mut guard) = self.data.lock() {
            *guard = data;
        }
        
        Ok(())
    }

    fn save_data(&self) -> Result<()> {
        let path = Path::new(&self.file_path);
        
        // Create directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        
        let writer = BufWriter::new(file);
        
        if let Ok(guard) = self.data.lock() {
            serde_json::to_writer_pretty(writer, &*guard)?;
        }
        
        Ok(())
    }
}

impl EmbeddingStorage for LocalFileStorage {
    fn store_embedding(&mut self, record: EmbeddingRecord) -> Result<()> {
        if let Ok(mut guard) = self.data.lock() {
            guard.insert(record.id.clone(), record);
        }
        self.save_data()?;
        Ok(())
    }

    fn get_embedding(&self, id: &str) -> Result<Option<EmbeddingRecord>> {
        if let Ok(guard) = self.data.lock() {
            Ok(guard.get(id).cloned())
        } else {
            Ok(None)
        }
    }

    fn get_all_embeddings(&self) -> Result<Vec<EmbeddingRecord>> {
        if let Ok(guard) = self.data.lock() {
            Ok(guard.values().cloned().collect())
        } else {
            Ok(Vec::new())
        }
    }

    fn delete_embedding(&mut self, id: &str) -> Result<bool> {
        let deleted = if let Ok(mut guard) = self.data.lock() {
            guard.remove(id).is_some()
        } else {
            false
        };
        
        if deleted {
            self.save_data()?;
        }
        
        Ok(deleted)
    }
}

pub fn open_temp_storage() -> Result<Box<dyn EmbeddingStorage>> {
    let path = format!("workshop_local_{}.json", Uuid::new_v4());
    let storage = LocalFileStorage::new(path)?;
    Ok(Box::new(storage))
}

pub fn store_dummy(storage: &mut Box<dyn EmbeddingStorage>, name: &str, embedding_len: usize) -> Result<String> {
    // Create an EmbeddingRecord with the given name and an embedding of the specified length
    // Following the pattern from app/src/register.rs
    let id = Uuid::new_v4().to_string();
    
    // Create a dummy embedding vector filled with zeros
    let embedding = vec![0.0f32; embedding_len];
    
    // Create metadata HashMap
    let mut metadata = HashMap::new();
    metadata.insert("type".to_string(), "dummy".to_string());
    metadata.insert("embedding_length".to_string(), embedding_len.to_string());
    
    let record = EmbeddingRecord {
        id: id.clone(),
        name: name.to_string(),
        embedding,
        created_at: chrono::Utc::now(),
        metadata,
    };
    
    // Store the embedding record
    storage.store_embedding(record)?;
    
    // Return the generated id
    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_test() -> Result<()> {
        let mut storage = open_temp_storage()?;

        // Store two dummy embeddings
        let id1 = store_dummy(&mut storage, "alice", 8)?;
        let id2 = store_dummy(&mut storage, "bob", 16)?;

        // Retrieve and check alice
        let loaded1 = storage.get_embedding(&id1)?.expect("alice should exist");
        assert_eq!(loaded1.name, "alice");
        assert_eq!(loaded1.embedding.len(), 8);
        assert_eq!(loaded1.metadata.get("type").map(|s| s.as_str()), Some("dummy"));

        // Retrieve and check bob
        let loaded2 = storage.get_embedding(&id2)?.expect("bob should exist");
        assert_eq!(loaded2.name, "bob");
        assert_eq!(loaded2.embedding.len(), 16);
        assert_eq!(loaded2.metadata.get("embedding_length").map(|s| s.as_str()), Some("16"));

        // Get all embeddings
        let all = storage.get_all_embeddings()?;
        let names: Vec<_> = all.iter().map(|r| r.name.as_str()).collect();
        assert!(names.contains(&"alice"));
        assert!(names.contains(&"bob"));
        assert_eq!(all.len(), 2);

        // Delete alice
        let deleted = storage.delete_embedding(&id1)?;
        assert!(deleted, "alice should be deleted");

        // alice should no longer exist
        let should_be_none = storage.get_embedding(&id1)?;
        assert!(should_be_none.is_none());

        // bob should still exist
        let still_bob = storage.get_embedding(&id2)?;
        assert!(still_bob.is_some());

        // Delete bob
        let deleted_bob = storage.delete_embedding(&id2)?;
        assert!(deleted_bob, "bob should be deleted");

        // Now storage should be empty
        let all_after = storage.get_all_embeddings()?;
        assert!(all_after.is_empty());

        // Deleting a non-existent id returns false
        let deleted_none = storage.delete_embedding("nonexistent_id")?;
        assert!(!deleted_none);

        Ok(())
    }
}


