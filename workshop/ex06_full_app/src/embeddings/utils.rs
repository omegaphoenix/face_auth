use candle_core::{DType, Device, Tensor};
use candle_nn::{Module, VarBuilder, Func};
use anyhow::Result;
use candle_transformers::models::{convnext};


pub fn compute_embeddings(model: &Func, image: &Tensor) -> Result<Tensor> {
    // Check if input is a single image (3D: [C, H, W]) or batch (4D: [N, C, H, W])
    let input = if image.dims().len() == 3 {
        // Single image: add batch dimension
        image.unsqueeze(0)?
    } else {
        // Already batched or different format
        image.clone()
    };

    let input = input.to_dtype(DType::F16)?;
    let embeddings = model.forward(&input)?;
    Ok(embeddings.to_dtype(DType::F32)?)
}

pub fn build_model(model_name: &str) -> Result<Func> {
    let device = &Device::Cpu;
    let model_file = {
        let api = hf_hub::api::sync::Api::new()?;
        let api = api.model(model_name.to_string());
        api.get("model.safetensors")?
    };

    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[model_file], DType::F16, device)? };
    let model = convnext::convnext_no_final_layer(&convnext::Config::atto(), vb)?;

    Ok(model)
}
