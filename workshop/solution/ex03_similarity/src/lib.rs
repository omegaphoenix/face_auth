use anyhow::Result;
use candle_core::{DType, Device, Tensor};
use candle_nn::{Module, VarBuilder, Func};
use candle_transformers::models::convnext;

/// Normalize tensor using L2 normalization
fn normalize_l2(v: &Tensor) -> Result<Tensor> {
    Ok(v.broadcast_div(&v.sqr()?.sum_keepdim(1)?.sqrt()?)?)
}

/// Build the ConvNeXt model
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
