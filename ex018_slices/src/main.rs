/// Slices: Write a function that returns the first letter of a word
fn return_first_letter(st: &str)-> &str {
    &st[0..1]
}
fn main() {
    let res = return_first_letter("Aditya");
    println!("First letter of Aditya: {}", res);
}
