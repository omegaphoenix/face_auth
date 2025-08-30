//! src/camera/mod.rs
use anyhow::Result;
use std::time::{Duration, Instant};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use reqwest::blocking::Client;
use crate::config::*;
use crate::image_utils::imagenet::{self, image_with_std_mean};
use crate::embeddings::utils::compute_embeddings;
use candle_nn::Func;

use minifb::{Window, WindowOptions, Key};
use std::io::Read;
use image::{DynamicImage, ImageFormat};

pub fn capture_and_compute_average_embedding(model: &Func) -> Result<Vec<f32>> {
    println!("[*] Starting camera capture for embedding computation from: {{get_stream_url()}}");

    // Shared latest frame for display and sampling
    let latest_frame = Arc::new(Mutex::new(None::<Arc<DynamicImage>>));
    let latest_frame_clone_stream = Arc::clone(&latest_frame);
    let latest_frame_clone_display = Arc::clone(&latest_frame);

    // Channels to signal when threads should shutdown
    let (shutdown_tx_stream, shutdown_rx_stream) = mpsc::channel::<()>();
    let (shutdown_tx_display, shutdown_rx_display) = mpsc::channel::<()>();

    // Stream reader thread - just updates the latest frame
    let stream_handle = thread::spawn(move || {
        if let Err(e) = stream_reader(latest_frame_clone_stream, shutdown_rx_stream) {
            eprintln!("Stream reader error: {e}");
        }
    });

    // Display thread - shows the latest frame and listens for shutdown signal
    let display_handle = thread::spawn(move || {
        if let Err(e) = display_processor(latest_frame_clone_display, shutdown_rx_display) {
            eprintln!("Display processor error: {e}");
        }
    });

    // Main thread - samples frames for embedding computation
    let embedding_result = embedding_sampler_and_computer(model, latest_frame, shutdown_tx_stream, shutdown_tx_display);

    // Wait for threads to complete
    let _ = stream_handle.join();
    let _ = display_handle.join();

    embedding_result
}

fn stream_reader(
    latest_frame: Arc<Mutex<Option<Arc<DynamicImage>>>>,
    shutdown_rx: mpsc::Receiver<()>
) -> Result<()> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let mut response = client.get(get_stream_url()).send()?;
    let mut buffer = Vec::with_capacity(300_000);
    let chunk_size = get_chunk_size();
    let mut chunk_buffer = vec![0u8; chunk_size];
    let mut frame_count = 0;

    println!("Stream reader started...");

    loop {
        // Check for shutdown signal without blocking
        if shutdown_rx.try_recv().is_ok() {
            break;
        }

        match response.read(&mut chunk_buffer) {
            Ok(0) => {
                println!("Stream ended");
                break;
            }
            Ok(n) => {
                buffer.extend_from_slice(&chunk_buffer[..n]);

                // Process all available JPEG frames in buffer
                while let Some(jpeg_data) = extract_next_jpeg(&mut buffer) {
                    if let Ok(image) = decode_jpeg(&jpeg_data) {
                        // Update the shared latest frame
                        if let Ok(mut frame) = latest_frame.try_lock() {
                            *frame = Some(Arc::new(image));
                            frame_count += 1;

                            if frame_count % 100 == 0 {
                                println!("Processed {frame_count} frames");
                            }
                        }
                        // If mutex is locked, just skip this frame - no big deal
                    }
                }

                // Keep buffer size reasonable
                if buffer.len() > 200_000 {
                    // Avoid shrink/expand thrash; just clear and keep capacity
                    buffer.clear();
                }
            }
            Err(e) => {
                eprintln!("Stream read error: {e}");
                thread::sleep(Duration::from_millis(100));
            }
        }
    }

    println!("Stream reader finished");
    Ok(())
}

fn embedding_sampler_and_computer(
    model: &Func,
    latest_frame: Arc<Mutex<Option<Arc<DynamicImage>>>>,
    shutdown_tx_stream: mpsc::Sender<()>,
    shutdown_tx_display: mpsc::Sender<()>
) -> Result<Vec<f32>> {
    let mut sample_count = 0;
    let start_time = Instant::now();
    let mut processing_time_total = Duration::default();

    println!("Embedding sampler started - will process {} samples with {}ms intervals",
             get_num_images(), get_interval_millis());

    let mut collected_frames: Vec<Arc<DynamicImage>> = Vec::new();
    let mut processed_frames = Vec::new();

    // Collect all frames first
    while sample_count < get_num_images() {
        // Wait for the sampling interval
        thread::sleep(Duration::from_millis(get_interval_millis()));

        // Get the current latest frame
        let frame_to_process: Arc<DynamicImage> = {
            match latest_frame.lock() {
                Ok(frame_guard) => {
                    match frame_guard.as_ref() {
                        Some(frame) => Arc::clone(frame),
                        None => {
                            println!("No frame available yet, waiting...");
                            continue;
                        }
                    }
                }
                Err(_) => {
                    println!("Failed to lock frame mutex, skipping sample");
                    continue;
                }
            }
        };

        let processing_start = Instant::now();

        println!("[*] Collecting sample {} (elapsed: {:.2}s)",
                 sample_count + 1, start_time.elapsed().as_secs_f32());

        // Process frame for embedding computation
        
        let processed_frame =image_with_std_mean(
            &frame_to_process,
            224,
            &imagenet::IMAGENET_MEAN,
            &imagenet::IMAGENET_STD
        )?;

        // Store the processed frame and original frame
        processed_frames.push(processed_frame);
        collected_frames.push(frame_to_process);
        
        let processing_time = processing_start.elapsed();
        processing_time_total += processing_time;

        println!("[*] Sample {} processed and collected, processing_time: {:.3}s",
                sample_count + 1, processing_time.as_secs_f32());

        sample_count += 1;
    }

    // Signal both threads to shutdown after sampling is complete
    let _ = shutdown_tx_stream.send(());
    let _ = shutdown_tx_display.send(());

    // Now run inference once for all collected frames
    println!("[*] Running batch inference for {} samples (elapsed: {:.2}s)",
             processed_frames.len(), start_time.elapsed().as_secs_f32());
    
    let inference_start = Instant::now();
    let mut embeddings = Vec::new();
    
    // Stack all processed frames into a batch tensor and compute embeddings in one call
    {
        use candle_core::Tensor;
        // processed_frames is Vec<Tensor>
        let batch = Tensor::stack(&processed_frames, 0)?;
        println!("[*] Computing embeddings for batch of {} samples", processed_frames.len());
        let batch_embeddings = compute_embeddings(model, &batch)?;
        let batch_embeddings_vec = batch_embeddings.to_vec2::<f32>()?;
        for (i, embedding_vec) in batch_embeddings_vec.into_iter().enumerate() {
            println!("[*] Got embedding {} of {} (batch inference)", i + 1, processed_frames.len());
            embeddings.push(embedding_vec);
        }
    }
    
    let inference_time = inference_start.elapsed();
    println!("[*] Batch inference completed in {:.3}s (avg: {:.3}s per sample)",
             inference_time.as_secs_f32(), 
             inference_time.as_secs_f32() / embeddings.len() as f32);


    // Compute and return the average embedding
    if !embeddings.is_empty() {
        println!("[*] Computing average embedding from {{embeddings.len()}} samples");
        
        let embedding_length = embeddings[0].len();
        let mut avg_embedding = vec![0.0f32; embedding_length];
        
        // Sum all embeddings
        for embedding in &embeddings {
            for (i, &value) in embedding.iter().enumerate() {
                avg_embedding[i] += value;
            }
        }
        
        // Divide by number of embeddings to get average
        for value in &mut avg_embedding {
            *value /= embeddings.len() as f32;
        }
        
        let total_time = start_time.elapsed();
        let avg_processing_time = processing_time_total.as_secs_f32() / sample_count as f32;
        println!("Embedding sampler completed {} samples in {:.2}s (avg processing: {:.3}s per sample, inference: {:.3}s)",
                sample_count, total_time.as_secs_f32(), avg_processing_time, inference_time.as_secs_f32());

        Ok(avg_embedding)
    } else {
        Err(anyhow::anyhow!("No embeddings were generated"))
    }
}

fn display_processor(
    latest_frame: Arc<Mutex<Option<Arc<DynamicImage>>>>,
    shutdown_rx: mpsc::Receiver<()>
) -> Result<()> {
    const WIDTH: usize = 640;
    const HEIGHT: usize = 480;

    let mut window = Window::new("Live Stream", WIDTH, HEIGHT, WindowOptions::default())?;
    window.set_target_fps(30);

    println!("Display window opened. Press ESC to exit or wait for processing to complete.");

    let mut pixels: Vec<u32> = vec![0u32; WIDTH * HEIGHT];
    let black_pixels: Vec<u32> = vec![0u32; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Check if we should shutdown (non-blocking)
        if shutdown_rx.try_recv().is_ok() {
            break;
        }

        // Get the latest frame
        let current_frame: Option<Arc<DynamicImage>> = {
            match latest_frame.try_lock() {
                Ok(frame_guard) => frame_guard.as_ref().map(Arc::clone),
                Err(_) => None, // Mutex locked, use previous frame or black screen
            }
        };

        // Display current frame
        if let Some(frame) = current_frame {
            let img = frame.resize_exact(
                WIDTH as u32,
                HEIGHT as u32,
                image::imageops::FilterType::Nearest
            ).to_rgb8();

            for (dst, p) in pixels.iter_mut().zip(img.pixels()) {
                *dst = (p[0] as u32) << 16 | (p[1] as u32) << 8 | (p[2] as u32);
            }

            if let Err(e) = window.update_with_buffer(&pixels, WIDTH, HEIGHT) {
                eprintln!("Window update error: {e}");
                break;
            }
        } else {
            // Show black screen if no frame available yet
            if let Err(e) = window.update_with_buffer(&black_pixels, WIDTH, HEIGHT) {
                eprintln!("Window update error: {e}");
                break;
            }
        }
    }

    println!("Display window closed");
    Ok(())
}

// JPEG extraction functions
fn extract_next_jpeg(buffer: &mut Vec<u8>) -> Option<Vec<u8>> {
    let start_pos = find_jpeg_start(buffer)?;
    let end_pos = find_jpeg_end(buffer, start_pos)?;

    if end_pos < start_pos || end_pos >= buffer.len() {
        return None;
    }

    let jpeg_data = buffer[start_pos..=end_pos].to_vec();
    buffer.drain(..=end_pos);
    Some(jpeg_data)
}

fn find_jpeg_start(buffer: &[u8]) -> Option<usize> {
    // Manual byte scan to avoid extra overhead from iterator/window machinery
    let len = buffer.len();
    let mut i = 0;
    while i + 1 < len {
        if buffer[i] == 0xFF && buffer[i + 1] == 0xD8 {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn find_jpeg_end(buffer: &[u8], start_pos: usize) -> Option<usize> {
    if start_pos + 2 >= buffer.len() {
        return None;
    }

    let len = buffer.len();
    let mut i = start_pos + 2;
    while i + 1 < len {
        if buffer[i] == 0xFF && buffer[i + 1] == 0xD9 {
            return Some(i + 1);
        }
        i += 1;
    }
    None
}

fn decode_jpeg(jpeg_data: &[u8]) -> Result<DynamicImage> {
    if jpeg_data.len() < 10 {
        return Err(anyhow::anyhow!("JPEG data too small"));
    }

    Ok(image::load_from_memory_with_format(jpeg_data, ImageFormat::Jpeg)?)
}

