use candle_core::Device;
use std::io::{self, Write};

mod image_utils;
use crate::image_utils::imagenet;
mod embeddings;
mod config;
use embeddings::embeddings::build_model;
mod register;
use register::register;
use candle_nn::Func;
use std::sync::Arc;

pub fn main() -> anyhow::Result<()> {
    println!("Face Authentication System");
    println!("Available commands:");
    println!("  /register - Register a new user");
    println!("  /login - Login with existing user");
    println!("  /quit - Exit the application");
    println!("Enter a command:");

    let device = Device::Cpu;
    let model = build_model("timm/convnext_atto.d2_in1k")?;
    


    loop {
        // Print prompt
        print!("> ");
        io::stdout().flush()?;

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        // Trim whitespace and newlines
        let command = input.trim();

        // Process commands
        match command {
            "register" => {
                println!("Register command detected!");
                // TODO: Implement registration logic
                handle_register(&model)?;
            }
            "login" => {
                println!("Login command detected!");
                // TODO: Implement login logic
                handle_login()?;
            }
            "quit" | "exit" => {
                println!("Goodbye!");
                break;
            }
            "" => {
                // Empty input, continue loop
                continue;
            }
            _ => {
                println!("Unknown command: {}", command);
                println!("Available commands: /register, /login, /quit");
            }
        }
    }

    Ok(())
}

fn handle_register(model: &Func) -> anyhow::Result<()> {
    println!("Registration process started...");
    register(&model);
    Ok(())
}

fn handle_login() -> anyhow::Result<()> {
    println!("Login process started...");
    // TODO: Add login implementation
    // This could involve:
    // - Capturing user's face image
    // - Generating embeddings
    // - Comparing with stored embeddings
    Ok(())
}