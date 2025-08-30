use anyhow::Result;
use candle_core::{Device, DType, Tensor};
use image::{DynamicImage};

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
    let img = img
        .resize_to_fill(
            res as u32,
            res as u32,
            image::imageops::FilterType::Triangle,
        );
    let img = img.to_rgb8();
    let data = img.into_raw();
    let data = Tensor::from_vec(data, (res, res, 3), &Device::Cpu)?.permute((2, 0, 1))?;
    let mean = Tensor::new(mean, &Device::Cpu)?.reshape((3, 1, 1))?;
    let std = Tensor::new(std, &Device::Cpu)?.reshape((3, 1, 1))?;
    Ok((data.to_dtype(DType::F32)? / 255.)?
        .broadcast_sub(&mean)?
        .broadcast_div(&std)?)
}
