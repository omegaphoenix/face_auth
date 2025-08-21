use anyhow::Result;
use candle_core::{Device, DType, Error, Tensor};

/// Exercise goal: implement image loading + ImageNet normalization.
/// Steps:
/// - open image path with `image::ImageReader`
/// - resize to 224x224 (Triangle filter)
/// - convert to RGB8, then to a Tensor of shape (3, 224, 224)
/// - convert to f32 in [0,1]
/// - subtract mean and divide by std using ImageNet constants
pub fn load_and_normalize(_path: &str, mean: &[f32; 3],
    std: &[f32; 3],   res: usize,) -> Result<Tensor> {
    unimplemented!("TODO: implement image loading + normalization as described in the comments")
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn tensor_values_are_normalized() -> Result<()> {
        
        
        // ImageNet normalization constants
        let imagenet_mean = [0.485, 0.456, 0.406];
        let imagenet_std = [0.229, 0.224, 0.225];

        let t = load_and_normalize("../../app/test_images/brad1.png", &imagenet_mean, &imagenet_std, 224)?;
        
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
        assert!(min_val >= theoretical_min - 0.5, 
                "Minimum value {} is too low, should be >= {} (ImageNet normalized range)", 
                min_val, theoretical_min - 0.5);
        assert!(max_val <= theoretical_max + 0.5, 
                "Maximum value {} is too high, should be <= {} (ImageNet normalized range)", 
                max_val, theoretical_max + 0.5);
        
        // For additional verification, check that we're not just getting zeros or ones
        assert!(min_val < -0.5, "Values seem not normalized (min too high): {}", min_val);
        assert!(max_val > 0.5, "Values seem not normalized (max too low): {}", max_val);
        
        Ok(())
    }
}


