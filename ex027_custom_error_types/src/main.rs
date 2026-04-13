use std::fmt;

/// Custom Error Types: Create a custom error type implementing Error
#[derive(Debug)]
enum ParseError {
    EmptyInput,
    InvalidChar(char)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter)-> fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "Empty input space"),
            ParseError::InvalidChar(c) => write!(f, "Invalid charater: {}", c)
        }
    }
}

impl std::error::Error for ParseError {}

impl From<std::num::ParseIntError> for ParseError {
    fn from(_:std::num::ParseIntError)-> Self {
        ParseError::InvalidChar('0')
    }
}

fn parse_number(s: &str)-> std::result::Result<i32, ParseError> {
    if s.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    let num = s.parse()?;
    Ok(num)
}

fn parse_2(s: &str)-> std::result::Result<i32, ParseError> {
    if s.is_empty() {
        return Err(ParseError::EmptyInput);
    }
    
    match s.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(ParseError::InvalidChar(s.chars().next().unwrap()))
    }
    
}
fn main() {
    let res = parse_number("1");
    println!("Value obtained from string parsing: {:?}", res);

    let res = parse_number("abc");
    println!("Value obtained from string parsing: {:?}", res);

    let res = parse_number("");
    println!("Value obtained from string parsing: {:?}", res);

    let res = parse_2("1");
    println!("Value obtained from string parsing: {:?}", res);

    let res = parse_2("abc");
    println!("Value obtained from string parsing: {:?}", res);

    let res = parse_2("");
    println!("Value obtained from string parsing: {:?}", res);
    
}
