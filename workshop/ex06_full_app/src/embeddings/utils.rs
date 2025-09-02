use candle_core::{DType, Device, Tensor};
use candle_nn::{Module, VarBuilder, Func};
use anyhow::Result;
use candle_transformers::models::{convnext};


pub fn compute_embeddings(model: &Func, image: &Tensor) -> Result<Tensor> {
    // TODO: Exercise 02 - ConvNeXt Model & Embedding Generation
    //
    // Implement the embedding computation function that:
    // 1. Handles input format - check if input is single image or batch
    // 2. Adds batch dimension if needed (single image: [C,H,W] -> [1,C,H,W])
    // 3. Converts input tensor to appropriate data type for the model
    // 4. Runs forward pass through the ConvNeXt model
    // 5. Returns the feature embeddings as tensors
    //
    // Key operations needed:
    // - Tensor dimension checking and manipulation
    // - Data type conversion for model compatibility
    // - Model forward pass execution
    // - Output tensor processing
    //
    // Expected behavior:
    // - Single image input: [3, 224, 224] -> [1, 3, 224, 224] -> [1, 768]
    // - Batch input: [N, 3, 224, 224] -> [N, 3, 224, 224] -> [N, 768]
    
    todo!("Implement embedding computation from Exercise 02")
}

pub fn build_model(model_name: &str) -> Result<Func> {
    // TODO: Exercise 02 - ConvNeXt Model Loading - Use F16 data type
    //
    // Implement the model loading function that:
    // 1. Downloads the ConvNeXt model from Hugging Face Hub
    // 2. Loads the model weights using SafeTensors format
    // 3. Creates ConvNeXt architecture without the final classification layer
    // 4. Returns the model as a callable function
    //
    // Key operations needed:
    // - Use Hugging Face Hub API to download model files
    // - Load model weights with VarBuilder from SafeTensors
    // - Create ConvNeXt architecture using the appropriate config
    // - Remove final classification layer to get feature embeddings
    //
    // Model details:
    // - Model: "timm/convnext_atto.d2_in1k" (ConvNeXt-Atto variant)
    // - Input: 224x224x3 RGB images (ImageNet normalized)
    // - Output: 768-dimensional embedding vectors
    // - Format: SafeTensors for efficient loading
    //
    // Why remove final layer?
    // - Original model trained for ImageNet classification (1000 classes)
    // - We want feature representations (embeddings), not class predictions
    // - Feature vectors capture facial characteristics for similarity comparison
    
    todo!("Implement model loading from Exercise 02")
}
