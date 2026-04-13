use my_trait_b::HelloWorld;
use my_derive_b::HelloWorld;

#[derive(HelloWorld)]
#[HelloWorldName = "the big Pancakes"]
struct Rustacean;

fn main() {
    Rustacean::hello();    
}
