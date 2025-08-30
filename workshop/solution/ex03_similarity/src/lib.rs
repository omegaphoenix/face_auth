use anyhow::Result;
use candle_core::{Tensor};


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