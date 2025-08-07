#!/usr/bin/env python3
"""
Camera test script to help debug libcamera and OpenCV camera issues
"""

import os
import sys
import time
import cv2
import numpy as np
from dotenv import load_dotenv

# Load environment variables
load_dotenv('config.env')

def test_opencv_camera():
    """Test OpenCV camera"""
    print("Testing OpenCV camera...")
    
    device = int(os.getenv("OPENCV_DEVICE", "0"))
    print(f"Trying OpenCV device: {device}")
    
    cap = cv2.VideoCapture(device)
    if not cap.isOpened():
        print(f"❌ Failed to open OpenCV camera device {device}")
        return False
    
    print(f"✅ OpenCV camera opened successfully on device {device}")
    
    # Try to read a few frames
    for i in range(5):
        ret, frame = cap.read()
        if ret:
            print(f"✅ Frame {i+1}: {frame.shape}")
        else:
            print(f"❌ Failed to read frame {i+1}")
            cap.release()
            return False
    
    cap.release()
    print("✅ OpenCV camera test passed")
    return True

def test_libcamera():
    """Test libcamera"""
    print("Testing libcamera...")
    
    try:
        from picamera2 import Picamera2
        print("✅ picamera2 imported successfully")
    except ImportError as e:
        print(f"❌ Failed to import picamera2: {e}")
        print("Install with: pip install picamera2")
        return False
    
    try:
        camera = Picamera2()
        print("✅ Picamera2 object created")
        
        # Create configuration
        config = camera.create_preview_configuration(
            main={"size": (640, 480), "format": "RGB888"},
            buffer_count=6
        )
        camera.configure(config)
        print("✅ Camera configured")
        
        camera.start()
        print("✅ Camera started")
        
        # Wait for camera to stabilize
        time.sleep(2)
        
        # Try to capture frames
        for i in range(5):
            try:
                frame = camera.capture_array()
                if frame is not None and frame.size > 0:
                    print(f"✅ Frame {i+1}: {frame.shape}")
                else:
                    print(f"❌ Empty frame {i+1}")
                    camera.close()
                    return False
            except Exception as e:
                print(f"❌ Failed to capture frame {i+1}: {e}")
                camera.close()
                return False
        
        camera.close()
        print("✅ Libcamera test passed")
        return True
        
    except Exception as e:
        print(f"❌ Libcamera test failed: {e}")
        return False

def main():
    """Main test function"""
    print("=== Camera Test Script ===")
    print()
    
    camera_source = os.getenv("CAMERA_SOURCE", "opencv")
    print(f"Current camera source: {camera_source}")
    print()
    
    if camera_source.lower() == "libcamera":
        success = test_libcamera()
        if not success:
            print("\nTrying OpenCV as fallback...")
            success = test_opencv_camera()
    else:
        success = test_opencv_camera()
        if not success:
            print("\nTrying libcamera as fallback...")
            success = test_libcamera()
    
    print()
    if success:
        print("🎉 Camera test completed successfully!")
    else:
        print("❌ Camera test failed!")
        print("\nTroubleshooting tips:")
        print("1. Check if camera is connected and not in use by another application")
        print("2. For libcamera: Make sure you're on a Raspberry Pi with camera module")
        print("3. For OpenCV: Try different device numbers (0, 1, 2, etc.)")
        print("4. Check camera permissions")
        print("5. Try running with sudo if on Linux")

if __name__ == "__main__":
    main()
