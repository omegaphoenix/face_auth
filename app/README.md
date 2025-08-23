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
  name: "timm/convnext_atto.d2_in1k"     # Model name
  embedding_size: 768                     # Embedding vector size
```

## File Structure

```
src/
├── main.rs              # Main application entry point
├── config.rs            # Configuration management
├── register.rs          # Face registration logic
├── login.rs             # Face authentication logic
├── storage/             # Storage implementations
│   ├── mod.rs          # Storage trait and types
│   └── local_file.rs   # Local file storage
├── embeddings/          # Embedding computation
│   └── embeddings.rs    # Model and embedding logic
├── image_utils/         # Image processing utilities
│   └── imagenet.rs      # ImageNet preprocessing
└── camera/              # Camera integration
    └── mod.rs           # Camera capture logic
```

## Dependencies

- **candle-core/candle-nn**: Neural network framework
- **serde/serde_yaml**: Configuration serialization
- **reqwest**: HTTP client for video streaming
- **image**: Image processing
- **minifb**: Window management for display

## Troubleshooting

### Video Stream Issues

- Ensure the video stream URL is accessible
- Check network connectivity
- Verify the stream format is supported

### Storage Issues

- Ensure write permissions to the configured file path
- Check that the directory exists or can be created

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License.
