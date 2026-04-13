// Mutable References: Modify data through mutable references
fn main() {
    let mut a = String::from("Data not mutated");
    let b = &mut a;
    b.push_str(" But got modified!!!!");

    println!("Value of a: {}", a);
}
