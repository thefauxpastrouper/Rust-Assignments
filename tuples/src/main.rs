// Tuples: Create a tuple with mixed types and destructure it
fn main() {
    let mixed_types = (3.14, 1, true, 'R');
    let (a,b,c,d) = mixed_types;
    
    println!("a, b, c, and d: {}, {}, {}, and {}", a,b,c,d);
}
