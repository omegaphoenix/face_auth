use anyhow::Result;
use candle_nn::{Module, VarBuilder, Func};
use candle_transformers::models::convnext;
use candle_core::{DType, Device, Tensor};

/// Exercise: Build the ConvNeXt model from the main app and compute a single embedding.
pub fn build_model() -> Result<Func<'static>> {
    unimplemented!("TODO: build the convnext model using candle_transformers::models::convnext")
}

pub fn compute_embedding(model: &Func, image: &Tensor) -> Result<Tensor> {
    unimplemented!("TODO: call compute_embeddings on a preprocessed image tensor")
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::Device;
    use face_auth::image_utils::imagenet;

    #[test]
    fn embedding_computes() -> Result<()> {
        let model = build_model()?;
        let img = imagenet::load_image224("../../app/test_images/brad1.png")?.to_device(&Device::Cpu)?;
        let emb = compute_embedding(&model, &img)?;
        assert_eq!(emb.dims()[0], 1);
        Ok(())
    }
}


