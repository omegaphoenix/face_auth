# Exercise 02: ConvNeXt Model and Embedding Generation

## Overview

This exercise teaches you how to load a pre-trained ConvNeXt model and use it to generate face embeddings. You'll implement two key functions: `build_model()` to load the model and `compute_embedding()` to generate feature vectors from facial images.

## What is ConvNeXt?

ConvNeXt (Convolution meets NeXt) is a modern convolutional neural network architecture that bridges the gap between traditional CNNs and Vision Transformers (ViTs). Introduced by Facebook AI Research in 2022, ConvNeXt modernizes the standard ResNet architecture by incorporating design choices inspired by Vision Transformers.

### Key Features of ConvNeXt:
- **Pure Convolutional Architecture**: Uses only convolutions, no self-attention mechanisms
- **Modernized ResNet Design**: Incorporates macro and micro design choices from ViTs
- **Competitive Performance**: Achieves performance comparable to Swin Transformers
- **Efficiency**: Maintains the computational efficiency of traditional CNNs

### ConvNeXt-Atto Variant:
We use **ConvNeXt-Atto**, an ultra-lightweight variant that provides excellent performance for face recognition tasks while being computationally efficient.

## What are Face Embeddings?

Embeddings are dense, low-dimensional vector representations that capture the essential characteristics of a face in numerical form.

### Purpose of Face Embeddings:
1. **Dimensionality Reduction**: Convert 224×224×3 images (~150K pixels) to compact vectors (~768 dimensions)
2. **Feature Extraction**: Capture essential facial characteristics (eye shape, nose structure, etc.)
3. **Similarity Computation**: Enable mathematical comparison between different faces
4. **Efficient Storage**: Store compact representations instead of full images

### Properties of Good Face Embeddings:
- **Discriminative**: Different people produce different embeddings
- **Robust**: Similar embeddings for the same person under different conditions
- **Compact**: Much smaller than original images
- **Comparable**: Can be compared using mathematical similarity metrics

## Your Tasks

### Task 1: Implement `build_model()`

```rust
pub fn build_model() -> Result<Func<'static>>
```

This function should:
1. **Download Model**: Use Hugging Face Hub API to get "timm/convnext_atto.d2_in1k"
2. **Load Weights**: Load the SafeTensors model file
3. **Create Model**: Build ConvNeXt without the final classification layer
4. **Return Function**: Return a callable model function

#### Implementation Steps:
```rust
// 1. Set up device (CPU for this exercise)
let device = &Device::Cpu;

// 2. Download model from Hugging Face Hub
let api = hf_hub::api::sync::Api::new()?;
let api = api.model("timm/convnext_atto.d2_in1k".to_string());
let model_file = api.get("model.safetensors")?;

// 3. Load weights into VarBuilder
let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[model_file], DType::F32, &device)? };

// 4. Create ConvNeXt model without final layer
let model = convnext::convnext_no_final_layer(&convnext::Config::atto(), vb)?;
```

### Task 2: Implement `compute_embedding()`

```rust
pub fn compute_embedding(model: &Func, image: &Tensor) -> Result<Tensor>
```

This function should:
1. **Handle Input Format**: Check if input is single image or batch
2. **Add Batch Dimension**: If single image (3D), add batch dimension to make it 4D
3. **Forward Pass**: Run the image through the model
4. **Return Embeddings**: Return the feature vectors

#### Implementation Steps:
```rust
// 1. Check input dimensions and add batch dimension if needed
let input = if image.dim(0)? == 3 {
    image.unsqueeze(0)?  // Add batch dimension: [3,224,224] -> [1,3,224,224]
} else {
    image.clone()        // Already batched: [N,3,224,224]
};

// 2. Forward pass through the model
let embeddings = model.forward(&input)?;
```

## Technical Details

### Model Architecture:
- **Input**: 224×224×3 RGB images (ImageNet normalized)
- **Output**: 768-dimensional embedding vectors
- **Weights**: Pre-trained on ImageNet dataset
- **Format**: SafeTensors for efficient loading

### Tensor Shapes:
- **Single Image Input**: `[3, 224, 224]` → `[1, 3, 224, 224]` (add batch dim)
- **Batch Input**: `[N, 3, 224, 224]` → `[N, 3, 224, 224]` (keep as is)
- **Output**: `[N, 768]` where N is batch size

### Key Dependencies:
- `hf_hub` - Download models from Hugging Face
- `candle_transformers::models::convnext` - ConvNeXt implementation
- `candle_nn::VarBuilder` - Load model weights

## Testing

The test verifies that:
- Model loads successfully from Hugging Face
- Embedding computation works with preprocessed images
- Output tensor has the correct batch dimension

Run the test with:
```bash
cargo test
```

## Expected Behavior

After successful implementation:
- `build_model()` downloads and loads the ConvNeXt-Atto model
- `compute_embedding()` processes images and returns 768-dimensional embeddings
- The model handles both single images and batches automatically

## Next Steps

After completing this exercise, you'll be ready to:
- Learn similarity computation between embeddings (Exercise 03)
- Understand how these embeddings enable face recognition
- Build storage systems for embedding databases (Exercise 04)

## References

- **ConvNeXt Paper**: [A ConvNet for the 2020s](https://arxiv.org/abs/2201.03545)
- **Hugging Face Model**: [timm/convnext_atto.d2_in1k](https://huggingface.co/timm/convnext_atto.d2_in1k)
- **Candle ConvNeXt**: [GitHub Implementation](https://github.com/huggingface/candle/blob/main/candle-transformers/src/models/convnext.rs)
