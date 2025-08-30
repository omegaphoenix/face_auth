use candle_core::{Device, Result, Tensor, DType};

pub const IMAGENET_MEAN: [f32; 3] = [0.485f32, 0.456, 0.406];
pub const IMAGENET_STD: [f32; 3] = [0.229f32, 0.224, 0.225];
use image::{DynamicImage};

pub fn image_with_std_mean(
    img: &DynamicImage,
    res: usize,
    mean: &[f32; 3],
    std: &[f32; 3],
) -> Result<Tensor> {
    // TODO: Exercise 01 - Image Processing & ImageNet Normalization
    // 
    // Implement the image preprocessing pipeline that:
    // 1. Resizes the input image to the specified resolution using Triangle filtering
    // 2. Converts to RGB8 format to ensure consistent color channels  
    // 3. Creates a tensor with shape (3, height, width) - channels first format
    // 4. Normalizes pixel values from [0-255] to [0-1] range
    // 5. Applies ImageNet standardization: (pixel/255 - mean) / std
    //
    // Key operations needed:
    // - Image resizing and format conversion
    // - Tensor creation from raw pixel data
    // - Dimension reordering (channels-first format)
    // - Mathematical operations for normalization
    // - Broadcasting for per-channel operations
    //
    // Expected output: Tensor with shape (3, res, res) and ImageNet-normalized values
    // Value range: Approximately [-2.12, 2.64] based on ImageNet constants
    
    todo!("Implement image preprocessing and ImageNet normalization from Exercise 01")
}