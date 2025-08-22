use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
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
}

// TODO: Implement the LocalFileStorage struct
// It should store and load records in a JSON file using serde

impl LocalFileStorage {
    pub fn new(file_path: String) -> Result<Self> {
        // TODO: Implement the constructor
        // Hint: Just store the file_path in the struct
        let _ = file_path; // remove after implementing
        unimplemented!("TODO: implement LocalFileStorage::new")
    }

    fn load_records(&self) -> Result<Vec<EmbeddingRecord>> {
        // TODO: Load records from the JSON file
        // Hint: 
        // 1. Check if file exists, return empty Vec if not
        // 2. Read file content as string
        // 3. Handle empty files
        // 4. Deserialize JSON to Vec<EmbeddingRecord>
        unimplemented!("TODO: implement load_records")
    }

    fn save_records(&self, records: &[EmbeddingRecord]) -> Result<()> {
        // TODO: Save records to the JSON file
        // Hint:
        // 1. Serialize records to JSON string (use serde_json::to_string_pretty)
        // 2. Write string to file
        let _ = records; // remove after implementing
        unimplemented!("TODO: implement save_records")
    }
}

// TODO: Implement the EmbeddingStorage trait for LocalFileStorage

impl EmbeddingStorage for LocalFileStorage {
    fn store_embedding(&mut self, record: EmbeddingRecord) -> Result<()> {
        // TODO: Add the record to existing records and save
        // Hint:
        // 1. Load existing records
        // 2. Add new record to the list
        // 3. Save the updated list
        let _ = record; // remove after implementing
        unimplemented!("TODO: implement store_embedding")
    }

    fn get_embedding(&self, id: &str) -> Result<Option<EmbeddingRecord>> {
        // TODO: Find and return the record with the given id
        // Hint:
        // 1. Load all records
        // 2. Find the record with matching id
        // 3. Return Some(record) if found, None if not
        let _ = id; // remove after implementing
        unimplemented!("TODO: implement get_embedding")
    }

    fn get_all_embeddings(&self) -> Result<Vec<EmbeddingRecord>> {
        // TODO: Return all records
        // Hint: Just call load_records
        unimplemented!("TODO: implement get_all_embeddings")
    }

    fn delete_embedding(&mut self, id: &str) -> Result<bool> {
        // TODO: Remove the record with the given id
        // Hint:
        // 1. Load existing records
        // 2. Keep track of initial length
        // 3. Remove records with matching id
        // 4. Save updated records if anything was removed
        // 5. Return true if something was deleted, false otherwise
        let _ = id; // remove after implementing
        unimplemented!("TODO: implement delete_embedding")
    }
}

pub fn open_temp_storage() -> Result<Box<dyn EmbeddingStorage>> {
    // TODO: Create a LocalFileStorage with a unique temporary filename
    // Hint: 
    // 1. Generate a unique filename using Uuid: format!("workshop_local_{}.json", Uuid::new_v4())
    // 2. Create a LocalFileStorage with that path
    // 3. Return it as a Box<dyn EmbeddingStorage>
    unimplemented!("TODO: implement open_temp_storage")
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


