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
    // TODO: Exercise 05 - Vector Similarity Search (Optional Enhancement)
    //
    // The current implementation manually computes similarity for each embedding.
    // As an optional enhancement, you could implement a more efficient approach:
    //
    // 1. Create a helper function that computes cosine similarity for Vec<f32>
    // 2. Implement a simple similarity search that finds the best match
    // 3. This would demonstrate the concepts from Exercise 05 in a simpler context
    //
    // For now, the manual approach below works fine for the face authentication system.
    
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
    // TODO: Exercise 03 - L2 Normalization
    //
    // Implement L2 normalization function that:
    // 1. Calculates the L2 norm (magnitude) of the vector
    // 2. Divides the vector by its norm to get unit length
    // 3. Handles tensor dimensions correctly for broadcasting
    //
    // Formula: normalized_vector = vector / ||vector||₂
    //
    // Key operations needed:
    // - Element-wise square (.sqr())
    // - Sum along appropriate dimension (.sum_keepdim())
    // - Square root (.sqrt())
    // - Broadcasting division (.broadcast_div())
    //
    // Why L2 normalization?
    // - Ensures all embeddings have unit length (magnitude = 1)
    // - Standardizes comparisons between different embeddings
    // - Makes cosine similarity equivalent to dot product
    // - Reduces sensitivity to lighting and scale variations
    
    todo!("Implement L2 normalization from Exercise 03")
}




pub fn cosine_similarity(emb_a: &Tensor, emb_b: &Tensor) -> Result<f32> {
    // TODO: Exercise 03 - Cosine Similarity Computation
    //
    // Implement cosine similarity function that:
    // 1. Normalizes both input embeddings using L2 normalization
    // 2. Computes the dot product using matrix operations
    // 3. Extracts the scalar similarity value from the result tensor
    //
    // Formula: cosine_similarity = (A · B) / (||A|| × ||B||)
    // For normalized vectors: cosine_similarity = A · B
    //
    // Key operations needed:
    // - Use normalize_l2() function on both embeddings
    // - Matrix multiplication (.matmul()) for dot product
    // - Tensor transpose (.transpose()) for proper dimensions
    // - Tensor squeezing (.squeeze()) to remove size-1 dimensions
    // - Scalar extraction (.to_vec0()) to get final f32 value
    //
    // Expected similarity ranges:
    // - Same person: 0.7 - 0.95 (high similarity)
    // - Different people: 0.2 - 0.6 (lower similarity)
    // - Identical images: ~1.0 (perfect similarity)
    //
    // Why cosine similarity?
    // - Measures direction, not magnitude of vectors
    // - Less sensitive to lighting variations
    // - Industry standard for face recognition
    // - Returns intuitive scores between -1 and 1
    
    todo!("Implement cosine similarity computation from Exercise 03")
}

