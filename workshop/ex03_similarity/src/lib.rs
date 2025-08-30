use anyhow::Result;
use candle_core::Tensor;
use candle_nn::Func;

// Import functions from ex02_embeddings solution
use ex02_embeddings_solution::{build_model, compute_embedding};

fn normalize_l2(v: &Tensor) -> Result<Tensor> {
    unimplemented!("TODO: normalize the tensor using L2 normalization")
}
/// Exercise: Implement cosine similarity using the app utilities and verify thresholds.
pub fn cosine_similarity(_emb_a: &Tensor, _emb_b: &Tensor) -> Result<f32> {
    unimplemented!("TODO: use face_auth::embeddings::embeddings::compute_similarity")
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::Device;
    use candle_nn::Func;
    use ex01_image_processing_solution::image_with_std_mean;

    #[test]
    fn same_person_higher_similarity() -> Result<()> {
        let device = &Device::Cpu;
        let reader1 = image::ImageReader::open("../../app/test_images/brad1.png")?;
        let image1 = reader1.decode()?;
        let reader2 = image::ImageReader::open("../../app/test_images/brad2.png")?;
        let image2 = reader2.decode()?;
        let reader3 = image::ImageReader::open("../../app/test_images/tom.png")?;
        let image3 = reader3.decode()?;

        let imagenet_mean: [f32; 3] = [0.485, 0.456, 0.406];
        let imagenet_std: [f32; 3] = [0.229, 0.224, 0.225];
        
        let img1 = image_with_std_mean(&image1, 224, &imagenet_mean, &imagenet_std)?;
        let img2 = image_with_std_mean(&image2, 224, &imagenet_mean, &imagenet_std)?;
        let img3 = image_with_std_mean(&image3, 224, &imagenet_mean, &imagenet_std)?;
        let model: Func = build_model()?;
        let e1 = compute_embedding(&model, &img1)?;
        let e2 = compute_embedding(&model, &img2)?;
        let e3 = compute_embedding(&model, &img3)?;
        let s_same = cosine_similarity(&e1, &e2)?;
        let s_diff = cosine_similarity(&e1, &e3)?;
        assert!(s_same > s_diff);
        Ok(())
    }
}


