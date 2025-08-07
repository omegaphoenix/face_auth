use candle_core::Device;
use std::io::{self, Write};

mod image_utils;

mod embeddings;
mod config;
use embeddings::embeddings::build_model;
mod register;
use register::register;
mod storage;
use candle_nn::Func;


pub fn main() -> anyhow::Result<()> {
    println!("Face Authentication System");
    println!("Available commands:");
    println!("  /register - Register a new user");
    println!("  /login - Login with existing user");
    println!("  /quit - Exit the application");
    println!("Enter a command:");

    let _device = Device::Cpu;
    let model = build_model(config::get_model_name())?;
    


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
                handle_login(&model)?;
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
    
    // Get user name
    print!("Enter user name: ");
    io::stdout().flush()?;
    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name)?;
    let user_name = user_name.trim();
    
    if user_name.is_empty() {
        println!("User name cannot be empty");
        return Ok(());
    }
    
    // Initialize storage
    let storage_config = config::get_storage_config();
    let mut storage = storage_config.create_storage()?;
    
    register(&model, &mut storage, user_name).map_err(|e| anyhow::anyhow!("Registration failed: {}", e))?;
    println!("Registration completed successfully!");
    Ok(())
}

fn handle_login(_model: &Func) -> anyhow::Result<()> {
    println!("Login process started...");
    
    // Get user name
    print!("Enter user name: ");
    io::stdout().flush()?;
    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name)?;
    let user_name = user_name.trim();
    
    if user_name.is_empty() {
        println!("User name cannot be empty");
        return Ok(());
    }
    
    // Initialize storage
    let storage_config = config::get_storage_config();
    let storage = storage_config.create_storage()?;
    
    // Get stored embeddings for this user
    let all_embeddings = storage.get_all_embeddings()?;
    let user_embeddings: Vec<_> = all_embeddings
        .into_iter()
        .filter(|record| record.name == user_name)
        .collect();
    
    if user_embeddings.is_empty() {
        println!("No registered embeddings found for user '{}'", user_name);
        return Ok(());
    }
    
    println!("Found {} registered embeddings for user '{}'", user_embeddings.len(), user_name);
    println!("Please look at the camera for authentication...");
    
    // TODO: Implement face capture and comparison
    // For now, just show the stored embeddings
    for (i, record) in user_embeddings.iter().enumerate() {
        println!("Embedding {}: ID={}, Created={}", 
                i + 1, record.id, record.created_at);
    }
    
    Ok(())
}