use anyhow::Result;
use candle_core::{DType, Device, Tensor};
use candle_nn::{Module, VarBuilder, Func};
use candle_transformers::models::convnext;

/// Normalize tensor using L2 normalization
fn normalize_l2(v: &Tensor) -> Result<Tensor> {
    Ok(v.broadcast_div(&v.sqr()?.sum_keepdim(1)?.sqrt()?)?)
}

/// Exercise: Build the ConvNeXt model from the main app and compute a single embedding.
/// Implement these using `face_auth::embeddings::embeddings` helpers.
pub fn build_model() -> Result<Func> {
    let device = &Device::Cpu;
    let model_file = {
        let api = hf_hub::api::sync::Api::new()?;
        let api = api.model("timm/convnext_atto.d2_in1k".to_string());
        api.get("model.safetensors")?
    };

    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[model_file], DType::F32, &device)? };
    let model = convnext::convnext_no_final_layer(&convnext::Config::atto(), vb)?;

    Ok(model)
}

pub fn compute_embedding(model: &Func, image: &Tensor) -> Result<Tensor> {
    // If image is not a batch, unsqueeze it, else use as is
    let input = if image.dim(0)? == 3 {
        image.unsqueeze(0)?
    } else {
        image.clone()
    };

    let embeddings = model.forward(&input)?;
    let norm_emb = normalize_l2(&embeddings)?;
    Ok(norm_emb)
}