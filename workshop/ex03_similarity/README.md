# Exercise 03: Similarity Computation

## Overview

This exercise focuses on computing similarity between face embeddings using different distance metrics. You'll learn how to implement cosine similarity, understand when to use different metrics, and apply normalization techniques for optimal performance.

## Mathematical Foundation

### L2 Normalization
**Formula**: `normalized_vector = vector / ||vector||₂`

L2 normalization is crucial for embedding comparison as it:
- **Standardizes Magnitude**: Ensures all embeddings have unit length
- **Improves Robustness**: Reduces sensitivity to lighting and scale variations
- **Enables Fair Comparison**: Focuses on directional relationships rather than magnitude
- **Optimizes Similarity Metrics**: Makes cosine similarity equivalent to dot product

### Normalization Implementation
```rust
pub fn l2_normalize(embedding: &Tensor) -> Result<Tensor> {
    let norm = embedding.sqr()?.sum_keepdim(D::Minus1)?.sqrt()?;
    embedding.broadcast_div(&norm)
}
```

## Cosine Similarity vs Euclidean Distance

When comparing face embeddings, two primary metrics are commonly used: **Cosine Similarity** and **Euclidean Distance**. Understanding their differences is crucial for effective face authentication systems.

### Cosine Similarity

**Formula**: `cosine_similarity = (A · B) / (||A|| × ||B||)`

**Key Characteristics**:
- **Range**: [-1, 1], where 1 = identical, 0 = orthogonal, -1 = opposite
- **Magnitude Invariant**: Only considers direction, not magnitude of vectors
- **Normalized**: Automatically handles different vector magnitudes
- **Angular Measurement**: Measures angle between vectors in high-dimensional space

**Advantages for Face Embeddings**:
- **Lighting Robustness**: Less affected by illumination changes that scale embedding magnitudes
- **Expression Tolerance**: Better handles facial expression variations
- **Consistent Scale**: Produces normalized similarity scores across different face sizes
- **Standard Practice**: Widely adopted in face recognition literature

**Use Cases**:
- Face verification/identification systems
- When embeddings may have varying magnitudes
- Scenarios with lighting or scale variations

### Euclidean Distance

**Formula**: `euclidean_distance = ||A - B||₂ = sqrt(Σ(aᵢ - bᵢ)²)`

**Key Characteristics**:
- **Range**: [0, ∞), where 0 = identical, larger values = more different
- **Magnitude Sensitive**: Considers both direction and magnitude differences
- **Geometric**: Measures straight-line distance in embedding space
- **Absolute Measurement**: Provides absolute distance rather than relative similarity

**Advantages for Face Embeddings**:
- **Intuitive Interpretation**: Smaller distance = more similar faces
- **Clustering**: Better for embedding clustering and visualization
- **Speed**: Simpler computation compared to cosine similarity
- **Threshold Setting**: Often easier to set meaningful distance thresholds

**Use Cases**:
- When embeddings are already L2-normalized
- Clustering face embeddings for analysis
- Real-time applications requiring fast computation

### Practical Comparison

| Aspect | Cosine Similarity | Euclidean Distance |
|--------|------------------|-------------------|
| **Normalization** | Built-in magnitude normalization | Requires pre-normalized embeddings |
| **Lighting Robustness** | High | Medium (depends on normalization) |
| **Speed** | Slower (division required) | Faster (simple subtraction/sqrt) |
| **Interpretability** | Similarity score [0,1] | Distance value [0,∞) |
| **Industry Standard** | Preferred for face recognition | Common for general ML tasks |

### Implementation Recommendations

**For Face Authentication Systems**:
1. **Use Cosine Similarity** when:
   - Working with raw embeddings of varying magnitudes
   - Handling diverse lighting conditions
   - Following established face recognition practices

2. **Use Euclidean Distance** when:
   - Embeddings are already L2-normalized
   - Speed is critical for real-time applications
   - Building clustering or visualization systems

**Best Practice**: Many systems use L2-normalized embeddings with either metric, as cosine similarity of normalized vectors equals `1 - euclidean_distance²/2`.

## Exercise Tasks

In this exercise, you will:

1. **Implement Cosine Similarity**: Create a function to compute cosine similarity between embeddings
2. **Apply L2 Normalization**: Normalize embeddings before comparison
3. **Test Similarity Thresholds**: Verify that same-person comparisons have higher similarity

### Key Functions to Implement:

```rust
pub fn cosine_similarity(emb_a: &Tensor, emb_b: &Tensor) -> Result<f32>
```
- Compute cosine similarity between two embedding vectors
- Handle both normalized and unnormalized embeddings
- Return similarity score in range [0,1] for face authentication

## Technical Implementation

### Similarity Pipeline:
1. **Input**: Two face embeddings from ConvNeXt model
2. **Normalization**: Apply L2 normalization if needed
3. **Similarity Computation**: Calculate cosine similarity or euclidean distance
4. **Threshold Comparison**: Compare against authentication threshold

### Authentication Thresholds:
- **High Security**: 0.8-0.9 cosine similarity
- **Balanced**: 0.6-0.8 cosine similarity  
- **High Recall**: 0.4-0.6 cosine similarity

## Next Steps

After completing this exercise, you'll be ready to:
- Store embeddings in databases (Exercise 04)
- Build retrieval systems for face recognition
- Implement real-time authentication pipelines
- Optimize similarity computation for production systems
