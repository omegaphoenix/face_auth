from fastapi import FastAPI, Response, Query
from fastapi.responses import StreamingResponse
import cv2
import numpy as np
import os
from typing import Optional
import logging
from dotenv import load_dotenv

# Load environment variables from config.env
load_dotenv('config.env')

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

app = FastAPI()

# Camera source configuration
CAMERA_SOURCE = os.getenv("CAMERA_SOURCE", "opencv")  # "opencv" or "libcamera"
OPENCV_DEVICE = int(os.getenv("OPENCV_DEVICE", "0"))
LIBCAMERA_DEVICE = os.getenv("LIBCAMERA_DEVICE", "/dev/video0")

# Global camera objects
opencv_camera = None
libcamera_camera = None

def init_opencv_camera():
    """Initialize OpenCV camera"""
    global opencv_camera
    try:
        opencv_camera = cv2.VideoCapture(OPENCV_DEVICE)
        if not opencv_camera.isOpened():
            logger.error(f"Failed to open OpenCV camera device {OPENCV_DEVICE}")
            return False
        logger.info(f"OpenCV camera initialized on device {OPENCV_DEVICE}")
        return True
    except Exception as e:
        logger.error(f"Error initializing OpenCV camera: {e}")
        return False

def init_libcamera():
    """Initialize libcamera using picamera2"""
    global libcamera_camera
    try:
        from picamera2 import Picamera2
        from picamera2.encoders import JpegEncoder
        from picamera2.outputs import FileOutput
        
        libcamera_camera = Picamera2()
        config = libcamera_camera.create_preview_configuration(
            main={"size": (640, 480)},
            buffer_count=4
        )
        libcamera_camera.configure(config)
        libcamera_camera.start()
        logger.info("Libcamera initialized successfully")
        return True
    except ImportError:
        logger.error("picamera2 not available. Please install it for libcamera support.")
        return False
    except Exception as e:
        logger.error(f"Error initializing libcamera: {e}")
        return False

def get_frame_opencv():
    """Get frame from OpenCV camera"""
    if opencv_camera is None:
        return None
    
    success, frame = opencv_camera.read()
    if not success:
        logger.warning("Failed to read frame from OpenCV camera")
        return None
    
    return frame

def get_frame_libcamera():
    """Get frame from libcamera"""
    if libcamera_camera is None:
        return None
    
    try:
        frame = libcamera_camera.capture_array()
        # Convert from RGB to BGR for OpenCV compatibility
        frame = cv2.cvtColor(frame, cv2.COLOR_RGB2BGR)
        return frame
    except Exception as e:
        logger.error(f"Error capturing frame from libcamera: {e}")
        return None

def get_frame():
    """Get frame from the configured camera source"""
    if CAMERA_SOURCE.lower() == "libcamera":
        return get_frame_libcamera()
    else:
        return get_frame_opencv()

def gen_frames():
    """Generate video frames"""
    while True:
        frame = get_frame()
        if frame is None:
            logger.warning("Failed to get frame from camera")
            continue
        
        try:
            ret, buffer = cv2.imencode('.jpg', frame, [cv2.IMWRITE_JPEG_QUALITY, 80])
            if not ret:
                logger.warning("Failed to encode frame")
                continue
            
            frame_bytes = buffer.tobytes()
            yield (b'--frame\r\n'
                   b'Content-Type: image/jpeg\r\n\r\n' + frame_bytes + b'\r\n')
        except Exception as e:
            logger.error(f"Error processing frame: {e}")
            continue

@app.on_event("startup")
async def startup_event():
    """Initialize camera on startup"""
    logger.info(f"Initializing camera with source: {CAMERA_SOURCE}")
    
    if CAMERA_SOURCE.lower() == "libcamera":
        if not init_libcamera():
            logger.error("Failed to initialize libcamera, falling back to OpenCV")
            if not init_opencv_camera():
                logger.error("Failed to initialize any camera source")
    else:
        if not init_opencv_camera():
            logger.error("Failed to initialize OpenCV camera")

@app.on_event("shutdown")
async def shutdown_event():
    """Clean up camera resources"""
    global opencv_camera, libcamera_camera
    
    if opencv_camera is not None:
        opencv_camera.release()
        logger.info("OpenCV camera released")
    
    if libcamera_camera is not None:
        libcamera_camera.close()
        logger.info("Libcamera closed")

@app.get("/")
def index():
    return {
        "message": "Camera streaming server is running.",
        "camera_source": CAMERA_SOURCE,
        "opencv_device": OPENCV_DEVICE if CAMERA_SOURCE.lower() == "opencv" else None,
        "libcamera_device": LIBCAMERA_DEVICE if CAMERA_SOURCE.lower() == "libcamera" else None
    }

@app.get("/video_feed")
def video_feed():
    return StreamingResponse(gen_frames(), media_type='multipart/x-mixed-replace; boundary=frame')

@app.get("/camera_info")
def camera_info():
    """Get information about the current camera configuration"""
    return {
        "camera_source": CAMERA_SOURCE,
        "opencv_device": OPENCV_DEVICE,
        "libcamera_device": LIBCAMERA_DEVICE,
        "opencv_available": opencv_camera is not None and opencv_camera.isOpened(),
        "libcamera_available": libcamera_camera is not None
    }

@app.get("/switch_camera")
def switch_camera(source: str = Query(..., description="Camera source: 'opencv' or 'libcamera'")):
    """Switch between camera sources"""
    global CAMERA_SOURCE
    
    if source.lower() not in ["opencv", "libcamera"]:
        return {"error": "Invalid camera source. Use 'opencv' or 'libcamera'"}
    
    # Clean up current camera
    if CAMERA_SOURCE.lower() == "opencv" and opencv_camera is not None:
        opencv_camera.release()
    elif CAMERA_SOURCE.lower() == "libcamera" and libcamera_camera is not None:
        libcamera_camera.close()
    
    # Initialize new camera
    CAMERA_SOURCE = source.lower()
    success = False
    
    if CAMERA_SOURCE == "libcamera":
        success = init_libcamera()
        if not success:
            logger.error("Failed to initialize libcamera")
    else:
        success = init_opencv_camera()
        if not success:
            logger.error("Failed to initialize OpenCV camera")
    
    return {
        "message": f"Switched to {CAMERA_SOURCE} camera",
        "success": success,
        "camera_source": CAMERA_SOURCE
    }
