// Borrowing: Fix ownership errors using references
fn main() {
    let a = String::from("ADITYA");
    {
        let b = &a;
        println!("Value of b: {}", b)
    }
    println!("Value of a: {}", a);
}

