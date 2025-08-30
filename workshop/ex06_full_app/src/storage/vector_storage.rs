use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::local_file_vector_storage::LocalFileVectorStorage;

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
    #[allow(dead_code)]
    fn get_embedding(&self, id: &str) -> Result<Option<EmbeddingRecord>>;
    fn get_all_embeddings(&self) -> Result<Vec<EmbeddingRecord>>;
    #[allow(dead_code)]
    fn delete_embedding(&mut self, id: &str) -> Result<bool>;
}

pub enum StorageType {
    LocalFile(String),
}

impl StorageType {
    pub fn create_storage(self) -> Result<Box<dyn EmbeddingStorage>> {
        match self {
            StorageType::LocalFile(path) => {
                let storage = LocalFileVectorStorage::new(path)?;
                Ok(Box::new(storage))
            }
        }
    }
}
