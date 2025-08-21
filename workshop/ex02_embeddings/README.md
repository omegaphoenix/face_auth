# Exercise 02: ConvNeXt and Embeddings

## Overview

This exercise focuses on building a ConvNeXt model and computing embeddings for face authentication. You'll learn how to load a pre-trained ConvNeXt model and use it to generate feature embeddings from facial images.

## What is ConvNeXt?

ConvNeXt (Convolution meets NeXt) is a modern convolutional neural network architecture that bridges the gap between traditional CNNs and Vision Transformers (ViTs). Introduced by Facebook AI Research in 2022, ConvNeXt modernizes the standard ResNet architecture by incorporating design choices inspired by Vision Transformers.

### Key Features of ConvNeXt:
- **Pure Convolutional Architecture**: Uses only convolutions, no self-attention mechanisms
- **Modernized ResNet Design**: Incorporates macro and micro design choices from ViTs
- **Competitive Performance**: Achieves performance comparable to Swin Transformers
- **Efficiency**: Maintains the computational efficiency of traditional CNNs

### ConvNeXt Variants:
- **ConvNeXt-T (Tiny)**: 28M parameters
- **ConvNeXt-S (Small)**: 50M parameters  
- **ConvNeXt-B (Base)**: 89M parameters
- **ConvNeXt-L (Large)**: 198M parameters
- **ConvNeXt-Atto**: Ultra-lightweight variant used in this exercise

## What are Embeddings?

Embeddings are dense, low-dimensional vector representations of high-dimensional data (like images). In the context of face authentication:

### Purpose of Embeddings:
1. **Dimensionality Reduction**: Convert high-resolution images to compact feature vectors
2. **Feature Extraction**: Capture essential facial characteristics in numerical form
3. **Similarity Computation**: Enable mathematical comparison between faces
4. **Efficient Storage**: Store compact representations instead of full images

### Properties of Good Face Embeddings:
- **Discriminative**: Different faces produce different embeddings
- **Robust**: Similar under lighting, pose, and expression variations
- **Normalized**: Often L2-normalized for consistent similarity computation
- **Compact**: Typically 512-2048 dimensions vs millions of pixels

## Exercise Tasks

In this exercise, you will:

1. **Build ConvNeXt Model**: Load a pre-trained ConvNeXt-Atto model from Hugging Face
2. **Compute Embeddings**: Process facial images to generate feature embeddings
3. **Apply Normalization**: Use L2 normalization for consistent similarity computation

### Key Functions to Implement:

```rust
pub fn build_model() -> Result<Func>
```
- Load the pre-trained ConvNeXt model from Hugging Face Hub
- Use the "timm/convnext_atto.d2_in1k" model variant
- Return a callable model function

```rust
pub fn compute_embedding(model: &Func, image: &Tensor) -> Result<Tensor>
```
- Process input image tensor through the model
- Extract raw embedding features from ConvNeXt layers
- Handle both single images and batched inputs

## Technical Implementation

### Model Loading:
- Uses Hugging Face Hub API to download pre-trained weights
- Loads model weights from SafeTensors format
- Configures ConvNeXt with no final classification layer (for embeddings)

### Image Processing Pipeline:
1. **Input**: Preprocessed image tensor (224x224, ImageNet normalized)
2. **Forward Pass**: Process through ConvNeXt layers
3. **Feature Extraction**: Extract final feature representations
4. **Output**: Raw embedding vectors ready for similarity computation

### Mathematical Foundation:
- **Feature Extraction**: Extract final feature representations from ConvNeXt layers
- **Embedding Output**: Dense vector representations (typically 512-2048 dimensions)
- **Preprocessing**: Images normalized using ImageNet statistics

## Sources and Further Reading

### Primary References:
- **ConvNeXt Paper**: [A ConvNet for the 2020s](https://arxiv.org/abs/2201.03545)
- **Candle ConvNeXt Implementation**: [GitHub - Hugging Face Candle ConvNeXt](https://github.com/huggingface/candle/blob/main/candle-transformers/src/models/convnext.rs)
- **Hugging Face Model Hub**: [ConvNeXt-Atto Model](https://huggingface.co/timm/convnext_atto.d2_in1k)

## Next Steps

After completing this exercise, you'll be ready to:
- Learn similarity computation between face embeddings (Exercise 03)
- Build embedding databases for face recognition
- Implement real-time face authentication systems
- Explore advanced embedding techniques and architectures
