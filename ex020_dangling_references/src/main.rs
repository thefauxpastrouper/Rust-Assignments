/// Dangling References: Identify and fix dangling reference issues
fn dangle()->String {
    let s = String::from("Hello");
    s
}

fn main() {
    let s = dangle();
    println!("Value from the dangle function: {}", s);
}
