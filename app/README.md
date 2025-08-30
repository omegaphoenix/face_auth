# Face Authentication System

A Rust-based face authentication system using local file storage for storing face embeddings.

## Features

- **Face Embedding Generation**: Uses ConvNeXt model to generate high-quality face embeddings
- **Local File Storage**: Stores embeddings in JSON format for simplicity
- **Real-time Face Registration**: Capture and store face embeddings from video stream
- **User Authentication**: Compare captured faces with stored embeddings
- **Configurable**: Easy configuration via YAML file

## Installation

1. Clone the repository
2. Install dependencies:
   ```bash
   cargo build
   ```

## Configuration

The system uses `config.yaml` for configuration:

### Storage Configuration

```yaml
storage:
  type: "local_file"
  local_file:
    path: "embeddings.json"
```

## Usage

### Running the Application

```bash
cargo run
```

### Commands

- `register` - Register a new user by capturing face embeddings
- `login` - Authenticate an existing user  
- `quit` or `exit` - Exit the application

**Note**: Commands are entered without the `/` prefix (e.g., type `register`, not `/register`)

### Registration Process

1. Run the `register` command
2. Enter a user name
3. Look at the camera while the system captures multiple face samples
4. The system will store embeddings in your configured storage

### Authentication Process

1. Run the `login` command
2. Enter your registered user name
3. Look at the camera for authentication
4. The system will compare your face with stored embeddings

## Storage

The system uses local file storage to store face embeddings in JSON format. This provides:

- **Simplicity**: No external dependencies required
- **Reliability**: Works offline and is easy to backup
- **Transparency**: Human-readable JSON format for debugging

## Configuration Options

### Stream Configuration

```yaml
stream:
  url: "http://localhost:8000/video_feed"  # Video stream URL
  num_images: 5                           # Number of samples to capture
  interval_millis: 10                     # Interval between samples
  chunk_size: 8192                        # Network chunk size
```

### Model Configuration

```yaml
model:
  name: "timm/convnext_atto.d2_in1k"     # Model name from Hugging Face
```

### UI Configuration

```yaml
ui:
  window_title: "Face Authentication"      # Display window title
  window_width: 800                        # Window width in pixels
  window_height: 600                       # Window height in pixels
```

## File Structure

```
src/
├── main.rs                              # Main application entry point
├── config.rs                            # Configuration management
├── register.rs                          # Face registration logic
├── login.rs                             # Face authentication logic
├── storage/                             # Storage implementations
│   ├── storage.rs                      # Storage module exports
│   ├── vector_storage.rs               # Storage trait and types
│   └── local_file_vector_storage.rs    # Local file storage implementation
├── embeddings/                          # Embedding computation
│   ├── embeddings.rs                   # Module exports
│   └── utils.rs                        # Model loading and embedding computation
├── image_utils/                         # Image processing utilities
│   ├── image_utils.rs                  # Module exports
│   └── imagenet.rs                     # ImageNet preprocessing
├── camera/                              # Camera integration
│   ├── camera.rs                       # Module exports
│   └── camera_interactions.rs          # Camera capture and streaming logic
└── config.yaml                         # Configuration file
```

## Dependencies

### Core Dependencies
- **candle-core/candle-nn**: Neural network framework for model inference
- **candle-transformers**: Pre-trained model implementations (ConvNeXt)
- **hf-hub**: Hugging Face Hub integration for model downloading
- **anyhow**: Error handling and propagation

### Data & Configuration
- **serde/serde_yaml/serde_json**: Serialization for config and storage
- **uuid**: Unique identifier generation for embeddings
- **chrono**: Timestamp handling for embedding records

### Camera & Streaming
- **reqwest**: HTTP client for video streaming
- **image**: Image processing and format handling
- **minifb**: Window management for live video display

### Utilities
- **clap**: Command line argument parsing (for examples)
- **dotenv**: Environment variable loading
- **lazy_static**: Static configuration management

## Prerequisites

### Camera Server Setup

Before running the face authentication system, you need to start the camera server:

1. **Navigate to camera server directory**:
   ```bash
   cd camera_server
   ```

2. **Install Python dependencies**:
   ```bash
   pip install -r requirements.txt
   ```

3. **Start the camera server**:
   ```bash
   python camera_stream_api.py
   ```

4. **Verify camera stream**: Open http://localhost:8000/video_feed in your browser

### System Requirements
- **Camera**: Webcam or external camera connected to your system
- **Python 3.7+**: For the camera server
- **Rust 1.70+**: For the main application

## Troubleshooting

### Camera Stream Issues
- **No camera detected**: Ensure your camera is connected and not used by other applications
- **Stream URL not accessible**: Verify the camera server is running on http://localhost:8000
- **Poor image quality**: Check camera positioning and lighting conditions
- **Connection timeout**: Ensure firewall isn't blocking localhost connections

### Storage Issues
- **Permission denied**: Ensure write permissions to the configured file path
- **Directory not found**: The system will auto-create directories as needed
- **Corrupted embeddings.json**: Delete the file to start fresh (will lose registered users)

### Model Loading Issues
- **Download failures**: Check internet connection for Hugging Face model downloads
- **Memory issues**: ConvNeXt-Atto is lightweight, but ensure sufficient RAM
- **Performance**: First run may be slower due to model download and compilation

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License.
