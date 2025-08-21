use anyhow::Result;
use face_auth::storage::{EmbeddingRecord, EmbeddingStorage, StorageType};
use uuid::Uuid;
use std::collections::HashMap;

pub fn open_temp_storage() -> Result<Box<dyn EmbeddingStorage>> {
    let path = format!("workshop_local_{}.json", Uuid::new_v4());
    StorageType::LocalFile(path).create_storage()
}

pub fn store_dummy(storage: &mut Box<dyn EmbeddingStorage>, name: &str, embedding_len: usize) -> Result<String> {
    // TODO: Create an EmbeddingRecord with the given name and an embedding of the specified length.
    // Hint: use Uuid for id and chrono::Utc::now() for created_at.
    // Then call `storage.store_embedding(record)` and return the id.
    let _ = (name, embedding_len); // remove after implementing
    unimplemented!("TODO: implement store_dummy")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn store_and_get() -> Result<()> {
        let mut storage = open_temp_storage()?;
        let id = store_dummy(&mut storage, "alice", 8)?;
        let loaded = storage.get_embedding(&id)?.expect("should exist");
        assert_eq!(loaded.name, "alice");
        assert_eq!(loaded.embedding.len(), 8);
        Ok(())
    }
}


