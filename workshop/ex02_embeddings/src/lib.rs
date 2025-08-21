use anyhow::Result;
use candle_core::Tensor;
use candle_nn::Func;

/// Exercise: Build the ConvNeXt model from the main app and compute a single embedding.
/// Implement these using `face_auth::embeddings::embeddings` helpers.
pub fn build_model() -> Result<Func> {
    unimplemented!("TODO: return model built from face_auth config")
}

pub fn compute_embedding(_model: &Func, _image: &Tensor) -> Result<Tensor> {
    unimplemented!("TODO: call compute_embeddings on a preprocessed image tensor")
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::Device;
    use face_auth::image_utils::imagenet;

    #[test]
    #[ignore]
    fn embedding_computes() -> Result<()> {
        let model = build_model()?;
        let img = imagenet::load_image224("../../app/test_images/brad1.png")?.to_device(&Device::Cpu)?;
        let emb = compute_embedding(&model, &img)?;
        assert_eq!(emb.dims()[0], 1);
        Ok(())
    }
}


