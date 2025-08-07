use candle_core::{Device, Result, Tensor, Error, DType};

pub const IMAGENET_MEAN: [f32; 3] = [0.485f32, 0.456, 0.406];
pub const IMAGENET_STD: [f32; 3] = [0.229f32, 0.224, 0.225];
use image::{DynamicImage};

/// Loads an image from disk using the image crate at the requested resolution,
/// using the given std and mean parameters.
/// This returns a tensor with shape (3, res, res). imagenet normalization is applied.

pub fn load_image_with_std_mean<P: AsRef<std::path::Path>>(
    p: P,
    res: usize,
    mean: &[f32; 3],
    std: &[f32; 3],
) -> Result<Tensor> {
    let img = image::ImageReader::open(p)?
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
    (data.to_dtype(DType::F32)? / 255.)?
        .broadcast_sub(&mean)?
        .broadcast_div(&std)
}

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
    (data.to_dtype(DType::F32)? / 255.)?
        .broadcast_sub(&mean)?
        .broadcast_div(&std)
}

/// Loads an image from disk using the image crate at the requested resolution.
/// This returns a tensor with shape (3, res, res). imagenet normalization is applied.
pub fn load_image<P: AsRef<std::path::Path>>(p: P, res: usize) -> Result<Tensor> {
    load_image_with_std_mean(p, res, &IMAGENET_MEAN, &IMAGENET_STD)
}

/// Loads an image from disk using the image crate, this returns a tensor with shape
/// (3, 224, 224). imagenet normalization is applied.
pub fn load_image224<P: AsRef<std::path::Path>>(p: P) -> Result<Tensor> {
    load_image(p, 224)
}

pub fn load_image112<P: AsRef<std::path::Path>>(p: P) -> Result<Tensor> {
    load_image(p, 112)
}

// use std::error::Error;
// use image::{DynamicImage, ImageFormat, imageops::resize, imageops::FilterType};
// use mediapipe_rs::tasks::vision::ImageEmbedderBuilder;



// /// Decode JPEG bytes into a DynamicImage
// pub fn decode_jpeg(buffer: &[u8]) -> Result<DynamicImage, Box<dyn Error>> {
//     let img = image::load_from_memory_with_format(buffer, ImageFormat::Jpeg)?;
//     Ok(img)
// }


// /// Save frame to disk as JPEG
// pub fn save_frame(frame: &DynamicImage, path: &str) -> Result<(), Box<dyn Error>> {
//     frame.save(path)?;
//     Ok(())
// }

// /// Compute a dummy embedding: average RGB of resized 32x32 image
// pub fn compute_image_embedding(img: &DynamicImage, task: &ImageEmbedder) -> Result<Vec<f32>, Box<dyn Error>> {
//     // Convert to RGB8 first, then resize to 32x32
//     let rgb_image = img.to_rgb8();
//     let resized = resize(&rgb_image, 32, 32, FilterType::Triangle);
//     let (width, height) = resized.dimensions();
//     let mut session = task.new_session()?;

//     let embeds = session.embed(&image::open(rgb_image)?)?;
//     Ok(embeds)
// }