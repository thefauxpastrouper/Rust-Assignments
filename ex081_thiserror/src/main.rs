// anyhow/thiserror: Use popular error handling crates
use thiserror::Error;
use std::fs;

#[derive(Error, Debug)]
enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("User Not Found: {id}")]
    NotFound { id: u32 }
}

fn trigger_io_error() -> Result<(), AppError> {
    let _content = fs::read_to_string("nonexistent.txt")?;
    Ok(())
}

fn trigger_parse_error() -> Result<(), AppError> {
    "not a number".parse::<i32>()?;
    Ok(())
}

fn trigger_not_found_error() -> Result<(), AppError> {
    Err(AppError::NotFound { id: 404 })
}

fn main() {
    println!("{:?}", trigger_io_error().unwrap_err());
    println!("{:?}", trigger_parse_error().unwrap_err());
    println!("{:?}", trigger_not_found_error().unwrap_err());    
}
