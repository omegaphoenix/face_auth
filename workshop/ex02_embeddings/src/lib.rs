use anyhow::Result;
#[allow(unused_imports)]
use candle_core::{DType, Device, Tensor};
#[allow(unused_imports)]
use candle_nn::{Func, Module, VarBuilder};
#[allow(unused_imports)]
use candle_transformers::models::convnext;

/// Exercise: Build the ConvNeXt model from the main app and compute a single embedding.
pub fn build_model() -> Result<Func<'static>> {
    // 1. Download Model: Use Hugging Face Hub API to get "timm/convnext_atto.d2_in1k"
    let api = hf_hub::api::sync::Api::new()?;
    let api = api.model("timm/convnext_atto.d2_in1k".to_string());

    // 2. Load Weights: Load the SafeTensors model file
    let model_file = api.get("model.safetensors")?;
    // Create VarBuilder from downloaded weights
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[model_file], DType::F32, &device)? };

    // 3. Create Model: Build ConvNeXt without the final classification layer
    let model = convnext::convnext_no_final_layer(&convnext::Config::atto(), vb)?;

    // 4. Return Function: Return a callable model function
    Ok(model)
}

pub fn compute_embedding(model: &Func, image: &Tensor) -> Result<Tensor> {
    // 1. Handle Input Format: Check if input is single image or batch
    // Handle batch dimensions
    let batched_input = if image.dim(0)? == 3 {
        // Single image (C,H,W)
        // 2. Add Batch Dimension: If needed, ensure proper tensor dimensions
        image.unsqueeze(0)? // Add batch: (1,C,H,W)
    } else {
        image.clone() // Already batched (N,C,H,W)
    };

    // 3. Forward Pass: Run the image through the model
    let embeddings = model.forward(&batched_input)?;

    // 4. Return Embeddings: Return the feature vectors
    Ok(embeddings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::Tensor;
    use ex01_image_processing_solution::image_with_std_mean;

    #[test]
    fn build_model_works() -> Result<()> {
        let model = build_model()?;
        // Create a dummy input tensor with the expected shape (1, 3, 224, 224)
        let dummy = Tensor::zeros(
            (1, 3, 224, 224),
            candle_core::DType::F32,
            &candle_core::Device::Cpu,
        )?;
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
