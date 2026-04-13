// Unsafe Traits: Implement unsafe traits for custom types
unsafe trait EvenValue{
    unsafe fn get_value(&self) -> i32;
}

#[derive(Debug)]
struct MyNumber(i32);

unsafe impl EvenValue for MyNumber {
    unsafe fn get_value(&self) -> i32 {
        &self.0 * 2
    }
}

fn main() {
    let number = MyNumber(23);
    let value = unsafe { number.get_value() };
    println!("Hello, world!, result: {:?}", value);
}
