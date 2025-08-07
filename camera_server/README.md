# Camera Streaming Server

A FastAPI-based camera streaming server that supports both OpenCV and libcamera as camera sources.

## Features

- **Multiple Camera Sources**: Support for both OpenCV and libcamera
- **Dynamic Camera Switching**: Switch between camera sources at runtime
- **Environment Configuration**: Easy configuration via environment variables
- **Error Handling**: Robust error handling with fallback options
- **Logging**: Comprehensive logging for debugging

## Installation

1. Install the required dependencies:
```bash
pip install -r requirements.txt
```

2. For libcamera support on Raspberry Pi, you may need to install additional system packages:
```bash
sudo apt update
sudo apt install python3-picamera2
```

## Configuration

You can configure the camera source using environment variables:

### Environment Variables

- `CAMERA_SOURCE`: Camera source to use (`opencv` or `libcamera`)
- `OPENCV_DEVICE`: OpenCV camera device number (default: 0)
- `LIBCAMERA_DEVICE`: Libcamera device path (default: /dev/video0)

### Example Configuration

```bash
# Use OpenCV camera
export CAMERA_SOURCE=opencv
export OPENCV_DEVICE=0

# Use libcamera
export CAMERA_SOURCE=libcamera
export LIBCAMERA_DEVICE=/dev/video0
```

Or create a `config.env` file and load it:
```bash
source config.env
```

## Usage

### Starting the Server

```bash
uvicorn camera_stream_api:app --host 0.0.0.0 --port 8000
```

### API Endpoints

- `GET /`: Get server status and camera information
- `GET /health`: Health check endpoint
- `GET /video_feed`: Stream video feed
- `GET /test_frame`: Test single frame capture (returns JPEG image)
- `GET /camera_info`: Get detailed camera configuration information
- `GET /switch_camera?source={opencv|libcamera}`: Switch between camera sources

### Examples

1. **View the video stream**:
   - Open your browser and go to `http://localhost:8000/video_feed`

2. **Get camera information**:
   ```bash
   curl http://localhost:8000/camera_info
   ```

3. **Switch to libcamera**:
   ```bash
   curl "http://localhost:8000/switch_camera?source=libcamera"
   ```

4. **Switch back to OpenCV**:
   ```bash
   curl "http://localhost:8000/switch_camera?source=opencv"
   ```

## Camera Sources

### OpenCV Camera
- Works with most USB cameras and webcams
- Cross-platform support
- Good performance on most systems

### Libcamera
- Optimized for Raspberry Pi cameras
- Better performance on Raspberry Pi hardware
- Requires `picamera2` library
- Only available on Linux systems with libcamera support

## Troubleshooting

### Testing Your Camera Setup
First, run the test script to diagnose camera issues:
```bash
python test_camera.py
```

This script will test both OpenCV and libcamera configurations and provide detailed feedback.

### Libcamera Issues
1. **Import Error**: Make sure `picamera2` is installed:
   ```bash
   pip install picamera2
   ```

2. **Permission Issues**: On Raspberry Pi, you might need to run with sudo or add your user to the video group:
   ```bash
   sudo usermod -a -G video $USER
   ```

3. **Device Not Found**: Check if your camera is properly connected and recognized:
   ```bash
   vcgencmd get_camera
   ```

4. **Frame Capture Failures**: If you see "Failed to get frame from camera" warnings:
   - The camera may need time to stabilize after startup
   - Try increasing the buffer count in config.env: `LIBCAMERA_BUFFER_COUNT=8`
   - Reduce frame rate: `LIBCAMERA_FRAME_RATE=15`
   - Check if another application is using the camera

### OpenCV Issues
1. **Camera Not Found**: Try different device numbers (0, 1, 2, etc.)
2. **Permission Issues**: Make sure your user has access to video devices

### General Issues
- Check the server logs for detailed error messages
- Ensure your camera is not being used by another application
- Try restarting the server after changing camera sources
- Use the `/health` endpoint to check camera status: `curl http://localhost:8000/health`
- Test single frame capture: `curl http://localhost:8000/test_frame -o test.jpg`

## Development

The server automatically falls back to OpenCV if libcamera initialization fails. This ensures the server remains functional even if libcamera is not available or properly configured.

## License

This project is open source and available under the MIT License.
