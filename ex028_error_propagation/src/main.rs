/// Error Propagation: Use ? operator for error propagation
fn parse_number(s: &str)-> std::result::Result<i32, std::num::ParseIntError> {
    let num = s.parse()?;
    Ok(num)
}
fn main() {
    match parse_number("abc") {
        Ok(num) => println!("Value found: {}", num),
        Err(e) => println!("Error Found!!! : {}", e)
    }
    match parse_number("1") {
        Ok(num) => println!("Value found: {}", num),
        Err(e) => println!("Error Found!!! : {}", e)
    }
    match parse_number("") {
        Ok(num) => println!("Value found: {}", num),
        Err(e) => println!("Error FOund!!! : {}", e)
    }    
}
