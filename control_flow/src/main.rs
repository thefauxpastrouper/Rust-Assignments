/// Control Flow: Write a function that returns "Even" or "Odd" based on input
fn find_even_or_odd(a: i32) -> String {
    if a % 2 == 0 {
        let res = String::from("Even");
        res
    }else {
        let res = String::from("Odd");
        res
    }
}

fn main() {
    let a = 2;
    let b = 3;

    println!("Check a and b: {} and {}", find_even_or_odd(a), find_even_or_odd(b))
}
