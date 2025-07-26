use std::error::Error;
use std::time::{Duration, Instant};
use reqwest::blocking::Client;
use crate::config::*;
use crate::image_utils::*;
use minifb::{Window, WindowOptions, Key};
use std::io::Read;

pub fn register() -> Result<(), Box<dyn Error>> {
    println!("[*] Registering from: {}", STREAM_URL);

    let mut response = Client::new().get(STREAM_URL).send()?;
    let mut sample_count = 0;
    let mut last_sample_time = Instant::now();

    let width = 640;
    let height = 480;
    let mut window = Window::new("Live", width, height, WindowOptions::default())?;

    let mut buffer = Vec::new();
    let mut chunk_buffer = [0u8; CHUNK_SIZE];

    println!("Starting to read stream...");
    while sample_count < NUM_IMAGES && window.is_open() && !window.is_key_down(Key::Escape) {
        match response.read(&mut chunk_buffer) {
            Ok(0) => break,
            Ok(n) => {
                buffer.extend_from_slice(&chunk_buffer[..n]);
                let mut latest_frame = None;
                while let Some(jpeg_data) = extract_next_jpeg(&mut buffer) {
                    if let Ok(frame) = decode_jpeg(&jpeg_data) {
                        latest_frame = Some(frame);
                    }
                }
                if let Some(frame) = latest_frame {
                    // Resize, convert and update window here
                    let img = frame.resize_exact(width as u32, height as u32, image::imageops::FilterType::Nearest).to_rgb8();
                    let pixels: Vec<u32> = img.pixels()
                        .map(|p| (p[0] as u32) << 16 | (p[1] as u32) << 8 | (p[2] as u32))
                        .collect();
                    window.update_with_buffer(&pixels, width, height)?;
    
                    if last_sample_time.elapsed() >= Duration::from_millis(INTERVAL_MILLIS) {
                        // Spawn embedding + save on a separate thread or async task if possible
                        let embedding = compute_dummy_embedding(&frame)?;
                        println!("[*] Sample {} embedding: {:?}", sample_count + 1, embedding);
                        save_frame(&frame, &format!("sample_{}.jpg", sample_count + 1))?;
                        sample_count += 1;
                        last_sample_time = Instant::now();
                    }
                }
    
                // Clean buffer aggressively if needed
                if buffer.len() > 200_000 {
                    buffer.clear();
                }
            }
            Err(e) => {
                eprintln!("Error reading stream: {}", e);
                break;
            }
        }
    }

    Ok(())
}

fn extract_next_jpeg(buffer: &mut Vec<u8>) -> Option<Vec<u8>> {
    let start_pos = find_jpeg_start(buffer)?;
    let end_pos = find_jpeg_end(buffer, start_pos)?;
    let jpeg_data = buffer[start_pos..=end_pos].to_vec();
    buffer.drain(..=end_pos);
    
    Some(jpeg_data)
}

fn find_jpeg_start(buffer: &[u8]) -> Option<usize> {
    for i in 0..buffer.len().saturating_sub(1) {
        if buffer[i] == 0xFF && buffer[i + 1] == 0xD8 {
            return Some(i);
        }
    }
    None
}

fn find_jpeg_end(buffer: &[u8], start_pos: usize) -> Option<usize> {
    for i in (start_pos + 2)..buffer.len().saturating_sub(1) {
        if buffer[i] == 0xFF && buffer[i + 1] == 0xD9 {
            return Some(i + 1);
        }
    }
    None
}
