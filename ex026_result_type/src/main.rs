use std::fs;
use std::io;

/// Result Type: Handle file operations using Result
fn read_line(path: &str)-> io::Result<String> {
    fs::read_to_string(path)
}
  
fn main() {
    match read_line("ex026_result_type/hello.txt") {
        Ok(content) => println!("Content: {}", content),
        Err(e) => eprintln!("Err: {}", e)
    }
}
