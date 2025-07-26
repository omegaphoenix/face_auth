use std::error::Error;
use image::{DynamicImage, ImageFormat, imageops::resize, imageops::FilterType};

/// Decode JPEG bytes into a DynamicImage
pub fn decode_jpeg(buffer: &[u8]) -> Result<DynamicImage, Box<dyn Error>> {
    let img = image::load_from_memory_with_format(buffer, ImageFormat::Jpeg)?;
    Ok(img)
}


/// Save frame to disk as JPEG
pub fn save_frame(frame: &DynamicImage, path: &str) -> Result<(), Box<dyn Error>> {
    frame.save(path)?;
    Ok(())
}

/// Compute a dummy embedding: average RGB of resized 32x32 image
pub fn compute_dummy_embedding(img: &DynamicImage) -> Result<Vec<f32>, Box<dyn Error>> {
    // Convert to RGB8 first, then resize to 32x32
    let rgb_image = img.to_rgb8();
    let resized = resize(&rgb_image, 32, 32, FilterType::Triangle);
    let (width, height) = resized.dimensions();

    let mut r = 0u64;
    let mut g = 0u64;
    let mut b = 0u64;

    for pixel in resized.pixels() {
        r += pixel[0] as u64;
        g += pixel[1] as u64;
        b += pixel[2] as u64;
    }

    let total = (width * height) as f32;

    Ok(vec![
        r as f32 / total,
        g as f32 / total,
        b as f32 / total,
    ])
}
