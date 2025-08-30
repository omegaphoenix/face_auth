use crate::storage::vector_storage::{EmbeddingRecord, EmbeddingStorage};
use candle_nn::Func;
use anyhow::Result;
use uuid::Uuid;
use crate::camera::camera_interactions::{capture_and_compute_average_embedding};

pub fn register(model: &Func, storage: &mut Box<dyn EmbeddingStorage>, user_name: &str) -> Result<()> {
    println!("[*] Registering user '{user_name}'");

    // Capture frames and compute the average embedding using the new camera module
    let avg_embedding = capture_and_compute_average_embedding(model)?;

    // Store the average embedding record
    let avg_record = EmbeddingRecord {
        id: Uuid::new_v4().to_string(),
        name: user_name.to_string(),
        embedding: avg_embedding.clone(),
        created_at: chrono::Utc::now(),
        metadata: {
            let mut meta = std::collections::HashMap::new();
            meta.insert("type".to_string(), "average".to_string());
            meta.insert("sample_count".to_string(), "unknown".to_string()); // This info is now in the camera module
            meta
        },
    };

    if let Err(e) = storage.store_embedding(avg_record) {
        eprintln!("Failed to store average embedding: {e}");
        Err(e)
    } else {
        println!("[*] Stored average embedding for user '{}' (length: {})", 
                user_name, avg_embedding.len());
        Ok(())
    }
}
