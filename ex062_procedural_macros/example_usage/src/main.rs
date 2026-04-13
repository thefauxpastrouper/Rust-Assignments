use my_trait::HelloWorld;
use my_derive::HelloWorld;

#[derive(HelloWorld)]
struct Rustacean;

fn main() {
    Rustacean::hello();    
}
