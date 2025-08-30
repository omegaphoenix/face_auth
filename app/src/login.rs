use candle_nn::Func;
use crate::storage::vector_storage::{EmbeddingStorage, EmbeddingRecord};
use candle_core::Tensor;
use anyhow::Result;
use candle_core::Device;
use crate::camera::camera_interactions::{capture_and_compute_average_embedding};

pub fn login(model: &Func, storage: &dyn EmbeddingStorage, user_name: &str) -> Result<bool> {
    println!("[*] Attempting to login user '{user_name}'");

    // 1. Capture a new embedding from the camera
    let live_embedding = capture_and_compute_average_embedding(model)?;

    // 2. Retrieve all stored embeddings for the given user
    let all_embeddings = storage.get_all_embeddings()?;
    let user_embeddings: Vec<EmbeddingRecord> = all_embeddings
        .into_iter()
        .filter(|record| record.name == user_name)
        .collect();

    if user_embeddings.is_empty() {
        println!("[!] No registered embeddings found for user '{user_name}'");
        return Ok(false);
    }

    // 3. Compare the live embedding with each stored embedding
    let mut best_match_similarity = 0.0;

    let live_tensor = Tensor::new(live_embedding, &Device::Cpu)?.unsqueeze(0)?;
    for record in user_embeddings {   
        let stored_tensor = Tensor::new(record.embedding.as_slice(), &Device::Cpu)?.unsqueeze(0)?;
        let similarity = cosine_similarity(&live_tensor, &stored_tensor)?;
        println!("[*] Comparing with stored embedding (ID: {0}), similarity: {similarity:.4}", record.id);
        if similarity > best_match_similarity {
            best_match_similarity = similarity;
        }
    }


    let login_threshold = 0.7; 

    if best_match_similarity > login_threshold {
        println!("[+] Login successful for user '{user_name}' with similarity: {best_match_similarity:.4}");
        Ok(true)
    } else {
        println!("[!] Login failed for user '{user_name}'. Best similarity: {best_match_similarity:.4}");
        Ok(false)
    }
}


fn normalize_l2(v: &Tensor) -> Result<Tensor> {
    Ok(v.broadcast_div(&v.sqr()?.sum_keepdim(1)?.sqrt()?)?)
}




pub fn cosine_similarity(emb_a: &Tensor, emb_b: &Tensor) -> Result<f32> {
    let emb_a = normalize_l2(emb_a)?;
    let emb_b = normalize_l2(emb_b)?;
    let similarity = emb_a.matmul(&emb_b.transpose(0, 1)?)?;
    let similarity_value = similarity.squeeze(0)?.squeeze(0)?.to_vec0::<f32>()?;
    Ok(similarity_value)
}

