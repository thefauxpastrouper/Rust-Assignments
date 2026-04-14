use anyhow::{Context, Result};
use std::fs;

fn read_config() -> Result<String>{
    fs::read_to_string("config.json").context("Failed to read the config file")
}

fn main() -> Result<()>{
    let config = read_config().context("Failed to intialize the application")?;
    println!("{}",config);
    Ok(())
}
