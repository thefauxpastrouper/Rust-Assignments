use std::fs;
use std::num::ParseIntError;
#[allow(dead_code)]

#[derive(Debug)]
enum AppError {
    Io(std::io::Error), // Error observed: Error: Io(Os { code: 2, kind: NotFound, message: "No such file or directory" })
    Parse(ParseIntError) // Error Observed: Error: Parse(ParseIntError { kind: InvalidDigit })
}

// Convert std::io::Error into AppError
impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self{
        AppError::Io(error)
    }
}

// Convert ParseIntError into AppError
impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> Self {
        AppError::Parse(error)
    }
}

fn read_and_parse(_filename: &str) -> Result<i32, AppError>{
    let content = fs::read_to_string("count.txt")?;
    let num = content.trim().parse()?;
    Ok(num)
}

fn main() {
    match read_and_parse("./count.txt") {
        Ok(num) => println!("Resultant Num: {}", num),
        Err(e) => println!("Error: {:?}", e)
    }
}
