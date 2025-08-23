pub mod local_file;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingRecord {
    pub id: String,
    pub name: String,
    pub embedding: Vec<f32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub metadata: HashMap<String, String>,
}

pub trait EmbeddingStorage {
    fn store_embedding(&mut self, record: EmbeddingRecord) -> Result<()>;
    fn get_embedding(&self, id: &str) -> Result<Option<EmbeddingRecord>>;
    fn get_all_embeddings(&self) -> Result<Vec<EmbeddingRecord>>;
    fn delete_embedding(&mut self, id: &str) -> Result<bool>;
    fn search_similar(&self, embedding: &[f32], limit: usize) -> Result<Vec<(EmbeddingRecord, f32)>>;
}

pub enum StorageType {
    LocalFile(String),
}

impl StorageType {
    pub fn create_storage(self) -> Result<Box<dyn EmbeddingStorage>> {
        match self {
            StorageType::LocalFile(path) => {
                let storage = local_file::LocalFileStorage::new(path)?;
                Ok(Box::new(storage))
            }
        }
    }
}
