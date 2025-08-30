use super::vector_storage::{EmbeddingRecord, EmbeddingStorage};
use anyhow::Result;
use serde_json;
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::Mutex;

pub struct LocalFileVectorStorage {
    file_path: String,
    data: Mutex<HashMap<String, EmbeddingRecord>>,
}

impl LocalFileVectorStorage {
    pub fn new(file_path: String) -> Result<Self> {
        let storage = LocalFileVectorStorage {
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

impl EmbeddingStorage for LocalFileVectorStorage {
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
