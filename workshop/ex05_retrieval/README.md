# Exercise 05: Vector Retrieval and Similarity Search

## Overview

This exercise focuses on implementing vector retrieval functionality - the core component of any vector similarity search system. You'll build a top-k similarity search that finds the most similar embeddings to a query vector.

## The Approach

### What We're Building

The main goal is to implement the `top_k` function that:

1. Takes a query embedding vector
2. Compares it against all stored embeddings using cosine similarity
3. Returns the top-k most similar records, sorted by similarity score (descending)

### Implementation Strategy

```rust
pub fn top_k(storage: &dyn EmbeddingStorage, query: &[f32], k: usize) -> Result<Vec<(EmbeddingRecord, f32)>>
```

The algorithm should:

1. **Retrieve all embeddings** from storage
2. **Calculate similarity scores** between the query and each stored embedding using cosine similarity
3. **Sort by similarity** in descending order (highest similarity first)
4. **Return top-k results** respecting the k limit

### Key Considerations

- **Similarity Metric**: We use cosine similarity, which measures the angle between vectors (ignoring magnitude)
- **Performance**: This naive approach scans all embeddings - O(n) complexity
- **Memory**: Loads all embeddings into memory for comparison
- **Sorting**: Results must be sorted by similarity score in descending order

## Production Vector Databases

While this exercise teaches the fundamentals, production systems use specialized vector databases that provide:

- **Approximate Nearest Neighbor (ANN)** algorithms for sub-linear search time
- **Indexing strategies** (HNSW, IVF, LSH) for fast retrieval
- **Distributed storage** and horizontal scaling
- **Real-time updates** and deletions
- **Multiple similarity metrics** (cosine, euclidean, dot product)
- **Filtering and metadata queries**
- **Built-in persistence** and backup solutions

### Recommended Vector Databases

#### QDrant (Rust-Native) ⭐

**Perfect for Rust developers!** QDrant is written in Rust and provides excellent Rust client libraries.

- **Homepage**: [https://qdrant.tech/](https://qdrant.tech/)
- **Rust Client**: [https://github.com/qdrant/rust-client](https://github.com/qdrant/rust-client)
- **Documentation**: [https://qdrant.tech/documentation/](https://qdrant.tech/documentation/)
- **Features**: 
  - Native Rust implementation
  - HTTP and gRPC APIs
  - HNSW indexing
  - Payload filtering
  - Clustering support
  - Docker deployment

```toml
[dependencies]
qdrant-client = "1.7"
```

#### Other Popular Options

**Weaviate**
- **Homepage**: [https://weaviate.io/](https://weaviate.io/)
- **Rust Client**: [https://github.com/weaviate/weaviate-rust-client](https://github.com/weaviate/weaviate-rust-client)
- Features: GraphQL API, automatic vectorization, hybrid search

**Pinecone**
- **Homepage**: [https://www.pinecone.io/](https://www.pinecone.io/)
- **Rust Client**: [https://github.com/pinecone-io/pinecone-rust-client](https://github.com/pinecone-io/pinecone-rust-client)
- Features: Fully managed, serverless, real-time updates

**Chroma**
- **Homepage**: [https://www.trychroma.com/](https://www.trychroma.com/)
- **Rust Client**: [https://github.com/chroma-core/chroma-rs](https://github.com/chroma-core/chroma-rs)
- Features: Lightweight, embeddable, Python-first

**Milvus**
- **Homepage**: [https://milvus.io/](https://milvus.io/)
- **Rust Client**: [https://github.com/milvus-io/milvus-sdk-rust](https://github.com/milvus-io/milvus-sdk-rust)
- Features: Cloud-native, highly scalable, multiple index types

**pgvector (PostgreSQL Extension)**
- **Homepage**: [https://github.com/pgvector/pgvector](https://github.com/pgvector/pgvector)
- **Rust Client**: Use any PostgreSQL client like `tokio-postgres`
- Features: SQL-based, ACID transactions, familiar PostgreSQL ecosystem

## Learning Objectives

By implementing this exercise, you'll understand:

1. **Vector similarity search fundamentals**
2. **Trade-offs between accuracy and performance**
3. **Why specialized vector databases exist**
4. **How ranking and top-k selection works**
5. **The importance of proper similarity metrics**

## Implementation Tips

1. **Start simple**: Get the basic algorithm working first
2. **Handle edge cases**: Empty storage, k=0, k > number of records
3. **Test thoroughly**: Use the provided test cases to validate your implementation
4. **Consider performance**: Think about how this scales with dataset size
