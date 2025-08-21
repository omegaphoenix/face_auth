use anyhow::Result;
use face_auth::storage::{EmbeddingRecord, EmbeddingStorage, StorageType};
use uuid::Uuid;

pub fn open_temp_storage() -> Result<Box<dyn EmbeddingStorage>> {
    let path = format!("workshop_retrieval_{}.json", Uuid::new_v4());
    StorageType::LocalFile(path).create_storage()
}

pub fn add_record(storage: &mut Box<dyn EmbeddingStorage>, name: &str, embedding: Vec<f32>) -> Result<String> {
    // TODO: Construct an EmbeddingRecord with provided embedding and persist it.
    let _ = (name, &embedding);
    unimplemented!("TODO: implement add_record")
}

pub fn top_k(storage: &Box<dyn EmbeddingStorage>, query: &[f32], k: usize) -> Result<Vec<(EmbeddingRecord, f32)>> {
    // TODO: return the top-k most similar embeddings using storage.search_similar
    let _ = (storage, query, k);
    unimplemented!("TODO: implement top_k")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn retrieval_returns_best_match_first() -> Result<()> {
        let mut storage = open_temp_storage()?;
        let q = vec![1.0, 0.0, 0.0, 0.0];
        // close to q
        add_record(&mut storage, "near", vec![0.9, 0.0, 0.0, 0.0])?;
        // far from q
        add_record(&mut storage, "far", vec![0.0, 1.0, 0.0, 0.0])?;
        let res = top_k(&storage, &q, 1)?;
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].0.name, "near");
        Ok(())
    }
}


