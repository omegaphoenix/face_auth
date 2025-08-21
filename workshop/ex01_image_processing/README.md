# Exercise 1: Image Processing and Normalization

## Why Normalization is Needed

**ImageNet normalization is essential for three key reasons:**

1. **Neural Network Stability**: Raw pixel values (0-255) are too large and cause training instability. Normalizing to smaller ranges helps gradients flow properly during backpropagation.

2. **Pre-trained Model Compatibility**: Most computer vision models are trained on ImageNet-normalized data. Using the same normalization (mean=[0.485, 0.456, 0.406], std=[0.229, 0.224, 0.225]) ensures your input matches what the model expects.

3. **Feature Standardization**: Different color channels have different statistical distributions. Per-channel normalization (subtract mean, divide by std) centers each channel around zero with unit variance, giving equal importance to all color information.

Without proper normalization, pre-trained models will produce poor results because the input distribution doesn't match their training data.

## Implementation

This exercise implements:
- Image loading and resizing to 224×224
- Conversion to tensor format (3, 224, 224)
- Pixel value normalization (0-255 → 0-1)
- ImageNet standardization for model compatibility
