use anyhow::Result;
use candle_core::{DType, Device, Tensor};
use candle_nn::{Module, VarBuilder, Func};
use candle_transformers::models::convnext;

/// Normalize tensor using L2 normalization
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

/// Simple cosine similarity for Vec<f32> arrays
pub fn cosine_similarity_vec(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot_product / (norm_a * norm_b)
    }
}