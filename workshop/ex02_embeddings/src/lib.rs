use anyhow::Result;
#[allow(unused_imports)]
use candle_nn::{Module, VarBuilder, Func};
#[allow(unused_imports)]
use candle_transformers::models::convnext;
#[allow(unused_imports)]
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
    use candle_core::Tensor;

    #[test]
    fn build_model_works() -> Result<()> {
        let model = build_model()?;
        // Create a dummy input tensor with the expected shape (1, 3, 224, 224)
        let dummy = Tensor::zeros((1, 3, 224, 224), candle_core::DType::F32, &candle_core::Device::Cpu)?;
        // Try a forward pass
        let output = model.forward(&dummy)?;
        // Check output shape is [1, 768]
        let dims = output.dims();
        assert_eq!(dims.len(), 2, "Output tensor should have 2 dimensions");
        assert_eq!(dims[0], 1, "Batch dimension should be 1");
        println!("Output shape: {:?}", dims);
        assert_eq!(dims[1], 320, "Embedding dimension should be 768");
        Ok(())
    }

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
        assert_eq!(emb.dims()[1], 320, "Embedding dimension should be 768");
        Ok(())
    }
}
