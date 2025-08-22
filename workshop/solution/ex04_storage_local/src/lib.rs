use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use uuid::Uuid;

// Define the EmbeddingRecord struct locally (not imported)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingRecord {
    pub id: String,
    pub name: String,
    pub embedding: Vec<f32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub metadata: HashMap<String, String>,
}

// Define the EmbeddingStorage trait locally (not imported)
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

impl LocalFileStorage {
    pub fn new(file_path: String) -> Result<Self> {
        Ok(Self { file_path })
    }

    fn load_records(&self) -> Result<Vec<EmbeddingRecord>> {
        if !Path::new(&self.file_path).exists() {
            return Ok(Vec::new());
        }
        
        let content = fs::read_to_string(&self.file_path)?;
        if content.trim().is_empty() {
            return Ok(Vec::new());
        }
        
        let records: Vec<EmbeddingRecord> = serde_json::from_str(&content)?;
        Ok(records)
    }

    fn save_records(&self, records: &[EmbeddingRecord]) -> Result<()> {
        let content = serde_json::to_string_pretty(records)?;
        fs::write(&self.file_path, content)?;
        Ok(())
    }
}

impl EmbeddingStorage for LocalFileStorage {
    fn store_embedding(&mut self, record: EmbeddingRecord) -> Result<()> {
        let mut records = self.load_records()?;
        records.push(record);
        self.save_records(&records)
    }

    fn get_embedding(&self, id: &str) -> Result<Option<EmbeddingRecord>> {
        let records = self.load_records()?;
        Ok(records.into_iter().find(|r| r.id == id))
    }

    fn get_all_embeddings(&self) -> Result<Vec<EmbeddingRecord>> {
        self.load_records()
    }

    fn delete_embedding(&mut self, id: &str) -> Result<bool> {
        let mut records = self.load_records()?;
        let initial_len = records.len();
        records.retain(|r| r.id != id);
        
        if records.len() < initial_len {
            self.save_records(&records)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

pub fn open_temp_storage() -> Result<Box<dyn EmbeddingStorage>> {
    let path = format!("workshop_local_{}.json", Uuid::new_v4());
    let storage = LocalFileStorage::new(path)?;
    Ok(Box::new(storage))
}
