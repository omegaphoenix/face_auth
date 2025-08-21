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
    let img = image::ImageReader::open(_path)?
        .decode()
        .map_err(Error::wrap)?
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


