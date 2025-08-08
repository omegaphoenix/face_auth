use std::error::Error;
use candle_nn::Func;
use crate::storage::{EmbeddingStorage, EmbeddingRecord};
use crate::camera;

pub fn login(model: &Func, storage: &Box<dyn EmbeddingStorage>, user_name: &str) -> Result<bool, Box<dyn Error>> {
    println!("[*] Attempting to login user '{}'", user_name);

    // 1. Capture a new embedding from the camera
    let live_embedding = camera::capture_and_compute_average_embedding(model)?;

    // 2. Retrieve all stored embeddings for the given user
    let all_embeddings = storage.get_all_embeddings()?;
    let user_embeddings: Vec<EmbeddingRecord> = all_embeddings
        .into_iter()
        .filter(|record| record.name == user_name)
        .collect();

    if user_embeddings.is_empty() {
        println!("[!] No registered embeddings found for user '{}'", user_name);
        return Ok(false);
    }

    // 3. Compare the live embedding with each stored embedding
    let mut best_match_similarity = 0.0;
    for record in user_embeddings {
        let similarity = cosine_similarity(&live_embedding, &record.embedding);
        println!("[*] Comparing with stored embedding (ID: {}), similarity: {:.4}", record.id, similarity);
        if similarity > best_match_similarity {
            best_match_similarity = similarity;
        }
    }

    // TODO: Make this threshold configurable
    let login_threshold = 0.8; 

    if best_match_similarity > login_threshold {
        println!("[+] Login successful for user '{}' with similarity: {:.4}", user_name, best_match_similarity);
        Ok(true)
    } else {
        println!("[!] Login failed for user '{}'. Best similarity: {:.4}", user_name, best_match_similarity);
        Ok(false)
    }
}

fn cosine_similarity(v1: &[f32], v2: &[f32]) -> f32 {
    if v1.len() != v2.len() {
        return 0.0;
    }

    let dot_product: f32 = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum();
    let norm_v1: f32 = v1.iter().map(|a| a.powi(2)).sum::<f32>().sqrt();
    let norm_v2: f32 = v2.iter().map(|a| a.powi(2)).sum::<f32>().sqrt();

    if norm_v1 == 0.0 || norm_v2 == 0.0 {
        return 0.0;
    }

    dot_product / (norm_v1 * norm_v2)
}

