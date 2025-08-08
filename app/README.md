# Face Authentication System

A Rust-based face authentication system that supports both local file storage and Qdrant vector database for storing face embeddings.

## Features

- **Face Embedding Generation**: Uses ConvNeXt model to generate high-quality face embeddings
- **Multiple Storage Options**: 
  - Local JSON file storage (default)
  - Qdrant vector database storage
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

The system uses `config.yaml` for configuration. You can choose between two storage types:

### Local File Storage (Default)

```yaml
storage:
  type: "local_file"
  local_file:
    path: "embeddings.json"
```

### Qdrant Storage

```yaml
storage:
  type: "qdrant"
  qdrant:
    url: "http://localhost:6333"
    collection_name: "face_embeddings"
    api_key: null  # Set to your API key if using Qdrant Cloud
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

## Storage Options

### Local File Storage

- **Pros**: Simple, no external dependencies, works offline
- **Cons**: Limited scalability, no advanced search features
- **Best for**: Development, testing, small-scale deployments

### Qdrant Storage

- **Pros**: Scalable, advanced vector search, cloud-ready
- **Cons**: Requires Qdrant server setup
- **Best for**: Production deployments, large-scale applications

#### Setting up Qdrant

1. Install Qdrant:
   ```bash
   docker run -p 6333:6333 qdrant/qdrant
   ```

2. Update `config.yaml`:
   ```yaml
   storage:
     type: "qdrant"
     qdrant:
       url: "http://localhost:6333"
       collection_name: "face_embeddings"
   ```

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
├── storage/             # Storage implementations
│   ├── mod.rs          # Storage trait and types
│   ├── local_file.rs   # Local file storage
│   └── qdrant_storage.rs # Qdrant storage
├── embeddings/          # Embedding computation
│   └── embeddings.rs    # Model and embedding logic
└── image_utils/         # Image processing utilities
    └── imagenet.rs      # ImageNet preprocessing
```

## Dependencies

- **candle-core/candle-nn**: Neural network framework
- **qdrant-client**: Qdrant vector database client
- **serde/serde_yaml**: Configuration serialization
- **reqwest**: HTTP client for video streaming
- **image**: Image processing
- **minifb**: Window management for display

## Troubleshooting

### Qdrant Connection Issues

- Ensure Qdrant server is running on the configured URL
- Check firewall settings
- Verify API key if using Qdrant Cloud

### Video Stream Issues

- Ensure the video stream URL is accessible
- Check network connectivity
- Verify the stream format is supported

### Storage Issues

- For local file storage: Ensure write permissions to the configured path
- For Qdrant: Check collection creation permissions

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License.
