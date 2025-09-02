## Face Auth Workshop (1.5 hours)

Hands-on, modular exercises to learn the Face Auth Rust stack. Each exercise is a standalone crate with TODOs and unit tests. 
Work through them in order.

### Running
- From `workshop/`: `cargo test -p <exercise_crate>`
- To see hints in a file, search for `TODO`.

### Modules
1. ex01_image_processing: load and normalize images to tensors
2. ex02_embeddings: build model and generate embeddings
3. ex03_similarity: compute cosine similarity, thresholds
4. ex04_storage_local: store/retrieve embeddings via local JSON storage
5. ex05_retrieval: top_k retrieval based on cosine similarity

# Face Authentication Workshop

A comprehensive, hands-on workshop for learning face authentication systems using Rust, covering everything from model instantiation to complete system integration.

## 🎯 Workshop Overview

This workshop provides a deep dive into building face authentication systems through 5 progressive exercises. Each exercise focuses on a specific component while building towards a complete, production-ready system.

### Learning Objectives

- Understand machine learning model deployment in Rust
- Play with image preprocessing and computer vision pipelines
- Learn similarity computation and matching algorithms

### Workshop Structure





## 🚀 Getting Started

```bash
# Navigate to the workshop directory
cd workshop

# Install dependencies and compile
cargo build
```


### Threshold Guidelines (for similarity scores)

- **0.95+**: Very strict, minimal false positives
- **0.85-0.95**: Balanced for most applications
- **0.7-0.85**: Lenient, higher recall
- **<0.7**: Very permissive, may allow false positives


## 🔧 Troubleshooting

### Common Issues

**Slow Performance**:
- Enable release mode: `cargo run --release --bin exercise1_model`
- Use smaller models for development


## 📈 Performance Tuning

### Model Selection

<img width="2200" height="1800" alt="image" src="https://github.com/user-attachments/assets/8da7e6be-7a49-41e0-a6ec-c5a0abc11bb2" />


## 📚 Additional Resources

### Documentation

- [Candle Framework](https://github.com/huggingface/candle) - Deep learning framework
- [HuggingFace Models](https://huggingface.co/models) - Pre-trained model hub
- [Image Crate](https://docs.rs/image/) - Image processing library

### Research Papers

- "A ConvNet for the 2020s" - ConvNeXt architecture
- "FaceNet: A Unified Embedding for Face Recognition" - Face embedding concepts
- "Deep Face Recognition: A Survey" - Comprehensive overview

### Related Projects

- [OpenCV Rust](https://github.com/twistedfall/opencv-rust) - Computer vision
- [Qdrant](https://qdrant.tech/) - Vector database
- [Candle Examples](https://github.com/huggingface/candle/tree/main/candle-examples) - More ML examples

## 📄 License

This workshop is provided for educational purposes. Model weights are subject to their respective licenses from HuggingFace Hub.

---

**Happy Learning! 🚀**

*This workshop is designed to provide hands-on experience with face authentication systems. Take your time with each exercise and don't hesitate to experiment with different parameters and configurations.*
