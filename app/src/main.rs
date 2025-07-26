mod config;
mod image_utils;
mod register;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    register::register()
}