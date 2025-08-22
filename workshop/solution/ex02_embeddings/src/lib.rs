use anyhow::Result;
use candle_core::{DType, Device, Tensor};
use candle_nn::{Module, VarBuilder, Func};
use candle_transformers::models::convnext;

pub fn build_model() -> Result<Func<'static>> {
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
    Ok(embeddings)
}
