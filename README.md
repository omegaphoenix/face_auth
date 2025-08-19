# Face Auth

A comprehensive face authentication system built with Rust and Python that combines real-time camera streaming with advanced face recognition capabilities.

## Overview

Face Auth is a modular face authentication system consisting of two main components:

1. **App** - A Rust-based face authentication engine that handles face embedding generation, storage, and user authentication
2. **Camera Server** - A Python FastAPI server that provides camera streaming capabilities with support for multiple camera sources

## Features

- 🎯 **Real-time Face Authentication** - Fast face recognition using ConvNeXt models
- 📹 **Multiple Camera Sources** - Support for OpenCV, libcamera, and custom video streams
- 💾 **Flexible Storage** - Choose between local JSON storage or Qdrant vector database
- 🌐 **Web API** - RESTful camera streaming API with dynamic camera switching
- 🔧 **Easy Configuration** - YAML-based configuration for all components
- 🚀 **Cross-platform** - Works on Windows, Linux, and Raspberry Pi

## Architecture

```
┌─────────────────┐    HTTP Stream    ┌──────────────────┐
│   Camera Server │ ◄─────────────── │   Face Auth App  │
│   (Python)      │                  │   (Rust)         │
│                 │                  │                  │
│ • FastAPI       │                  │ • Face Detection │
│ • OpenCV        │                  │ • Embedding Gen  │
│ • libcamera     │                  │ • Storage        │
└─────────────────┘                  └──────────────────┘
         │                                      │
         ▼                                      ▼
┌─────────────────┐                  ┌──────────────────┐
│   USB Camera    │                  │     Storage      │
│   Raspberry Pi  │                  │ • Local JSON     │
│   Webcam        │                  │ • Qdrant DB      │
└─────────────────┘                  └──────────────────┘
```

## Quick Start

### Prerequisites

- **Rust** (1.70+) - [Install Rust](https://rustup.rs/)
- **Python** (3.8+) with pip
- **Camera** - USB webcam, built-in camera, or Raspberry Pi camera

### 1. Setup Camera Server

```bash
cd camera_server
pip install -r requirements.txt

# Configure camera source (optional)
cp config.env.example config.env
# Edit config.env to set your preferred camera

# Start the camera server
uvicorn camera_stream_api:app --host 0.0.0.0 --port 8000
```

### 2. Setup Face Auth App

```bash
cd app

# Build the application
cargo build --release

# Configure storage (optional)
cp config.yaml.example config.yaml
# Edit config.yaml to set your storage preferences

# Run the application
cargo run --release
```

### 3. Usage

1. **Register a new user:**
   - Run the app and type `register`
   - Enter a username
   - Look at the camera while the system captures face samples

2. **Authenticate:**
   - Type `login`
   - Enter your username
   - Look at the camera for authentication

## Components

### Face Auth App (Rust)

The core authentication engine built with Rust for performance and safety.

**Key Features:**
- ConvNeXt-based face embedding generation
- Support for local file and Qdrant vector database storage
- Real-time face capture and processing
- High-performance face matching algorithms

**Dependencies:**
- `candle-core` & `candle-nn` - Neural network framework
- `qdrant-client` - Vector database integration
- `reqwest` - HTTP client for video streaming
- `image` & `minifb` - Image processing and display

**Configuration:**
```yaml
# config.yaml
storage:
  type: "local_file"  # or "qdrant"
  local_file:
    path: "embeddings.json"
  qdrant:
    url: "http://localhost:6333"
    collection_name: "face_embeddings"

stream:
  url: "http://localhost:8000/video_feed"
  num_images: 5
  interval_millis: 10

model:
  name: "timm/convnext_atto.d2_in1k"
  embedding_size: 768
```

### Camera Server (Python)

A FastAPI-based streaming server that provides camera access with multiple source support.

**Key Features:**
- FastAPI web server with real-time streaming
- OpenCV and libcamera support
- Dynamic camera source switching
- Comprehensive error handling and logging
- Health check and diagnostic endpoints

**Dependencies:**
- `fastapi` & `uvicorn` - Web framework and server
- `opencv-python` - Computer vision library
- `picamera2` - Raspberry Pi camera support (optional)

**API Endpoints:**
- `GET /` - Server status and camera info
- `GET /health` - Health check
- `GET /video_feed` - Video stream
- `GET /camera_info` - Detailed camera configuration
- `GET /switch_camera?source={opencv|libcamera}` - Switch camera source

## Installation & Configuration

### Local File Storage (Default)

No additional setup required. Face embeddings are stored in `embeddings.json`.

**Pros:** Simple, no dependencies, works offline  
**Cons:** Limited scalability, no advanced search features  
**Best for:** Development, testing, small-scale deployments

### Qdrant Vector Database

For production deployments with scalable vector search capabilities.

```bash
# Start Qdrant with Docker
docker run -p 6333:6333 qdrant/qdrant

# Update app/config.yaml
storage:
  type: "qdrant"
  qdrant:
    url: "http://localhost:6333"
    collection_name: "face_embeddings"
```

**Pros:** Scalable, advanced vector search, cloud-ready  
**Cons:** Requires Qdrant server setup  
**Best for:** Production deployments, large-scale applications

### Camera Configuration

#### OpenCV (Default)
Works with most USB cameras and webcams across all platforms.

```bash
export CAMERA_SOURCE=opencv
export OPENCV_DEVICE=0
```

#### libcamera (Raspberry Pi)
Optimized for Raspberry Pi cameras with better performance.

```bash
# Install on Raspberry Pi
sudo apt update
sudo apt install python3-picamera2

export CAMERA_SOURCE=libcamera
export LIBCAMERA_DEVICE=/dev/video0
```

## Development

### Project Structure

```
Face Auth/
├── app/                    # Rust face authentication engine
│   ├── src/
│   │   ├── main.rs        # Application entry point
│   │   ├── config.rs      # Configuration management
│   │   ├── embeddings/    # Face embedding generation
│   │   ├── storage/       # Storage implementations
│   │   └── image_utils/   # Image processing utilities
│   ├── config.yaml        # App configuration
│   └── Cargo.toml         # Rust dependencies
├── camera_server/          # Python camera streaming server
│   ├── camera_stream_api.py # FastAPI application
│   ├── requirements.txt    # Python dependencies
│   └── config.env         # Environment configuration
└── mediapipe-rs/          # MediaPipe Rust library
```

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd Face\ Auth

# Setup Python virtual environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r camera_server/requirements.txt

# Build Rust application
cd app
cargo build --release
```

### Testing

```bash
# Test camera setup
cd camera_server
python test_camera.py

# Test face auth app
cd app
cargo test
cargo run --example storage_demo
```

## Troubleshooting

### Camera Issues

1. **Camera not detected:**
   ```bash
   # Test camera access
   python camera_server/test_camera.py
   ```

2. **Permission errors:**
   ```bash
   # Add user to video group (Linux)
   sudo usermod -a -G video $USER
   ```

3. **Raspberry Pi camera issues:**
   ```bash
   # Check camera detection
   vcgencmd get_camera
   ```

### Authentication Issues

1. **Poor recognition accuracy:**
   - Ensure good lighting conditions
   - Capture multiple face samples during registration
   - Keep face centered and looking at camera

2. **Storage connection issues:**
   - For Qdrant: Verify server is running and accessible
   - For local storage: Check file permissions

3. **Video stream errors:**
   - Verify camera server is running on correct port
   - Check network connectivity between components

### Performance Optimization

1. **Faster inference:**
   - Use release build: `cargo run --release`
   - Consider GPU acceleration if available
   - Adjust capture interval in config

2. **Memory usage:**
   - Limit number of face samples during capture
   - Use appropriate embedding dimensions
   - Consider batch processing for multiple users

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the individual component READMEs for details.

## Acknowledgments

- Built with [MediaPipe-rs](https://github.com/WasmEdge/mediapipe-rs) for computer vision tasks
- Uses [Candle](https://github.com/huggingface/candle) framework for neural network inference
- Camera streaming powered by [FastAPI](https://fastapi.tiangolo.com/)
- Vector storage with [Qdrant](https://qdrant.tech/)

## Support

For issues and questions:
- Check the [Troubleshooting](#troubleshooting) section
- Review component-specific READMEs in `app/` and `camera_server/`
- Open an issue in the repository

---

**Face Auth** - Secure, fast, and reliable face authentication for modern applications.
