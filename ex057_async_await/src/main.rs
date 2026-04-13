use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let content = "Hello String".to_string();
    
    fs::write("example.txt", content).await?;

    let data = fs::read_to_string("example.txt").await?;
    println!("File contents: {}", data);
    
    fs::remove_file("example.txt").await?;
    Ok(())
}
