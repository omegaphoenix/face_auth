# Exercise 1: Image Processing and Normalization

## Overview

This exercise teaches you how to properly preprocess images for computer vision models, specifically focusing on ImageNet normalization. You'll implement the `image_with_std_mean` function that transforms raw images into model-ready tensors.

## Why ImageNet Normalization is Critical

**ImageNet normalization is essential for three key reasons:**

1. **Neural Network Stability**: Raw pixel values (0-255) are too large and cause training instability. Normalizing to smaller ranges helps gradients flow properly during backpropagation.

2. **Pre-trained Model Compatibility**: Most computer vision models are trained on ImageNet-normalized data. Using the same normalization (mean=[0.485, 0.456, 0.406], std=[0.229, 0.224, 0.225]) ensures your input matches what the model expects.

3. **Feature Standardization**: Different color channels have different statistical distributions. Per-channel normalization (subtract mean, divide by std) centers each channel around zero with unit variance, giving equal importance to all color information.

Without proper normalization, pre-trained models will produce poor results because the input distribution doesn't match their training data.

## Your Task

Implement the `image_with_std_mean` function that:

1. **Resizes** the input image to the specified resolution using Triangle filtering
2. **Converts** to RGB8 format to ensure consistent color channels
3. **Creates** a tensor with shape `(3, height, width)` - channels first format
4. **Normalizes** pixel values from [0-255] to [0-1] range
5. **Applies** ImageNet standardization: `(pixel/255 - mean) / std`

## Implementation Steps

```rust
pub fn image_with_std_mean(
    img: &DynamicImage,
    res: usize,
    mean: &[f32; 3],
    std: &[f32; 3],
) -> Result<Tensor>
```

### Implementation Approach:

1. **Resize Image**: Use appropriate image resizing methods
2. **Convert Format**: Ensure consistent color channel format
3. **Extract Data**: Get raw pixel data from the image
4. **Create Tensor**: Build tensor with correct shape and dimensions
5. **Normalize**: Apply scaling and ImageNet standardization

### Key Operations Needed:
- Image resizing and format conversion
- Tensor creation from raw data
- Dimension reordering (channels-first format)
- Mathematical operations for normalization
- Broadcasting for per-channel operations

**Hint**: Check the CHEATSHEET.md for specific API calls and tensor operations.

## Testing

The test verifies that:
- Tensor values are in the expected normalized range (approximately [-2.5, 2.5])
- Values are actually normalized (not just zeros or ones)
- The transformation follows ImageNet standards

Run the test with:
```bash
cargo test
```

## Expected Output Format

- **Input**: DynamicImage of any size
- **Output**: Tensor with shape `(3, 224, 224)` and ImageNet-normalized values
- **Value Range**: Approximately [-2.12, 2.64] based on ImageNet constants

This preprocessing step is crucial for the face authentication pipeline, as it ensures images are in the exact format expected by the ConvNeXt model in the next exercise.
