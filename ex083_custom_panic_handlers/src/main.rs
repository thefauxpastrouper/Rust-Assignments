// Custom Panic Handlers: Implement custom panic behavior
use std::panic;

fn main() {
    panic::set_hook(Box::new(|info| {
        println!("Custom Panic Hook: {}", info);
    }));
    println!("Hello, world!");
    panic!("Something Went Wrong")
}
