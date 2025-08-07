use clap::{Parser, ValueEnum};
use candle_core::{DType, Device, Tensor};
use candle_nn::{Module, VarBuilder, Func};
use anyhow::Result;

use crate::imagenet;
use candle_transformers::models::convnext;

pub fn normalize_l2(v: &Tensor) -> Result<Tensor> {
    Ok(v.broadcast_div(&v.sqr()?.sum_keepdim(1)?.sqrt()?)?)
}

pub fn compute_embeddings(model: &Func, image: &Tensor) -> Result<Tensor> {
    let embeddings = model.forward(&image.unsqueeze(0)?)?;
    let norm_emb = normalize_l2(&embeddings)?;
    Ok(norm_emb)
}

pub fn build_model(model_name: &str) -> Result<Func> {
    let device = &Device::Cpu;
    let model_file = {
        let api = hf_hub::api::sync::Api::new()?;
        let api = api.model(model_name.to_string());
        api.get("model.safetensors")?
    };

    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[model_file], DType::F32, &device)? };
    let model = convnext::convnext_no_final_layer(&convnext::Config::atto(), vb)?;

    Ok(model)
}

pub fn compute_similarity(image_embedding1: &Tensor, image_embedding2: &Tensor) -> Result<f32> {
    let similarity = image_embedding1.matmul(&image_embedding2.transpose(0, 1)?)?;
    let similarity_value = similarity.squeeze(0)?.squeeze(0)?.to_vec0::<f32>()?;
    Ok(similarity_value)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_similarity() -> Result<()> {
        let device = &Device::Cpu;

        // Load two images of the same person
        let image_1 = imagenet::load_image224("test_images/brad1.png")?.to_device(&device)?;
        let image_2 = imagenet::load_image224("test_images/brad2.png")?.to_device(&device)?;

        // Build the model and compute embeddings
        let model = build_model("timm/convnext_atto.d2_in1k")?;
        let norm_emb_1 = compute_embeddings(&model, &image_1)?;
        let norm_emb_2 = compute_embeddings(&model, &image_2)?;

        // Calculate similarity using dot product
        let similarity = compute_similarity(&norm_emb_1, &norm_emb_2)?;
        
        // Since these are images of the same person, similarity should be high (close to 1.0)
        assert!(similarity > 0.7, "Similarity between same person images should be high, got {}", similarity);

        Ok(())
    }

    #[test]
    fn test_different_faces() -> Result<()> {
        let device = &Device::Cpu;

        // Load images of different people
        let image_1 = imagenet::load_image224("test_images/brad1.png")?.to_device(&device)?;
        let image_2 = imagenet::load_image224("test_images/tom.png")?.to_device(&device)?;

        // Build the model and compute embeddings
        let model = build_model("timm/convnext_atto.d2_in1k")?;
        let norm_emb_1 = compute_embeddings(&model, &image_1)?;
        let norm_emb_2 = compute_embeddings(&model, &image_2)?;

        // Calculate similarity using dot product
        let similarity = compute_similarity(&norm_emb_1, &norm_emb_2)?;
        
        // Since these are images of different people, similarity should be lower
        assert!(similarity < 0.7, "Similarity between different people should be low, got {}", similarity);

        Ok(())
    }
}