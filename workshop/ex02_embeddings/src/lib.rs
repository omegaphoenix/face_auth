use anyhow::Result;
use candle_nn::{Module, VarBuilder, Func};
use candle_transformers::models::convnext;
use candle_core::{DType, Device, Tensor};

/// Exercise: Build the ConvNeXt model from the main app and compute a single embedding.
pub fn build_model() -> Result<Func<'static>> {
    unimplemented!("TODO: build the convnext model using candle_transformers::models::convnext")
}

pub fn compute_embedding(_model: &Func, _image: &Tensor) -> Result<Tensor> {
    unimplemented!("TODO: call compute_embeddings on a preprocessed image tensor")
}

#[cfg(test)]
mod tests {
    use super::*;
    use ex01_image_processing_solution::image_with_std_mean;

    #[test]
    fn embedding_computes() -> Result<()> {
        let model = build_model()?;
        let reader = image::ImageReader::open("../../app/test_images/brad1.png")?;
        let image = reader.decode()?;
        let imagenet_mean: [f32; 3] = [0.485, 0.456, 0.406];
        let imagenet_std: [f32; 3] = [0.229, 0.224, 0.225];
        let img = image_with_std_mean(&image, 224, &imagenet_mean, &imagenet_std)?;
        let emb = compute_embedding(&model, &img)?;
        assert_eq!(emb.dims()[0], 1);
        Ok(())
    }
}


