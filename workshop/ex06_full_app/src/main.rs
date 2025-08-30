use candle_core::Device;
use std::io::{self, Write};

mod image_utils;

mod embeddings;
mod config;
mod camera;
use embeddings::utils::build_model;
mod login;
use login::login;
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
                println!("Unknown command: {command}");
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
    
    register(model, &mut storage, user_name).map_err(|e| anyhow::anyhow!("Registration failed: {}", e))?;
    println!("Registration completed successfully!");
    Ok(())
}

fn handle_login(model: &Func) -> anyhow::Result<()> {
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
    
    match login(model, &*storage, user_name) {
        Ok(true) => println!("Login successful!"),
        Ok(false) => println!("Login failed."),
        Err(e) => eprintln!("An error occurred during login: {e}"),
    }
    
    Ok(())
}