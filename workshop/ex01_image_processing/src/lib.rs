use anyhow::Result;
use candle_core::Tensor;
#[allow(unused_imports)]
use candle_core::{DType, Device};
use image::DynamicImage;

/// Exercise goal: implement image loading + ImageNet normalization.
/// Steps:
/// - open image path with `image::ImageReader`
/// - resize to (res, res) (Triangle filter)
/// - convert to RGB8, then to a Tensor of shape (3, res, rs)
pub fn image_with_std_mean(
    img: &DynamicImage,
    res: usize,
    mean: &[f32; 3],
    std: &[f32; 3],
) -> Result<Tensor> {
    // TODO: implement image loading + ImageNet normalization
    // 1. Resizes the input image to the specified resolution using Triangle filtering
    let resized_img = img.resize_to_fill(
        res as u32,
        res as u32,
        image::imageops::FilterType::Triangle,
    );

    // 2. Converts to RGB8 format to ensure consistent color channels
    let rgb8_img = resized_img.to_rgb8();

    // Extract raw pixel data
    let data: Vec<u8> = rgb8_img.into_raw(); // Returns Vec<u8> with RGB values

    // 3. Creates a tensor with shape (3, height, width) - channels first format
    let channels = 3;
    let tensor = Tensor::from_vec(data, (res, res, channels), &Device::Cpu)?;
    // Permute dimensions (e.g., height, width, channel (HWC) to CHW)
    let tensor = tensor.permute((2, 0, 1))?;

    // 4. Normalizes pixel values from [0-255] to [0-1] range
    // From array/slice
    // Scale values (e.g., 0-255 to 0-1)
    let normalized = tensor.to_dtype(DType::F32)? / 255.0;

    // 5. Applies ImageNet standardization: (pixel/255 - mean) / std
    let mean_tensor = Tensor::new(mean, &Device::Cpu)?;
    let reshaped_mean = mean_tensor.reshape((3, 1, 1))?;

    let std_tensor = Tensor::new(std, &Device::Cpu)?;
    let reshaped_std = std_tensor.reshape((3, 1, 1))?;

    Ok(normalized?
        .broadcast_sub(&reshaped_mean)?
        .broadcast_div(&reshaped_std)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tensor_values_are_normalized() -> Result<()> {
        // ImageNet normalization constants
        let imagenet_mean: [f32; 3] = [0.485, 0.456, 0.406];
        let imagenet_std: [f32; 3] = [0.229, 0.224, 0.225];
        let reader = image::ImageReader::open("../../app/test_images/brad1.png")?;
        let image = reader.decode()?;

        let t = image_with_std_mean(&image, 224, &imagenet_mean, &imagenet_std)?;

        // Check that tensor values are in reasonable range after normalization
        // Normalized values should typically be in range approximately [-2.5, 2.5]
        // since (0 - 0.485) / 0.229 ≈ -2.12 and (1 - 0.406) / 0.225 ≈ 2.64
        let tensor_data = t.flatten_all()?.to_vec1::<f32>()?;

        let min_val = tensor_data.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max_val = tensor_data.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));

        // Calculate theoretical bounds using ImageNet constants
        let theoretical_min = (0.0 - imagenet_mean[0]) / imagenet_std[0];
        let theoretical_max = (1.0 - imagenet_mean[2]) / imagenet_std[2];

        // Values should be in reasonable normalized range
        assert!(
            min_val >= theoretical_min - 0.5,
            "Minimum value {} is too low, should be >= {} (ImageNet normalized range)",
            min_val,
            theoretical_min - 0.5
        );
        assert!(
            max_val <= theoretical_max + 0.5,
            "Maximum value {} is too high, should be <= {} (ImageNet normalized range)",
            max_val,
            theoretical_max + 0.5
        );

        // For additional verification, check that we're not just getting zeros or ones
        assert!(
            min_val < -0.5,
            "Values seem not normalized (min too high): {}",
            min_val
        );
        assert!(
            max_val > 0.5,
            "Values seem not normalized (max too low): {}",
            max_val
        );

        Ok(())
    }
}
