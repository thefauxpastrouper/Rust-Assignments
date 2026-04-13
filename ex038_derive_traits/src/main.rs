/// Derive Traits: Use #[derive] for common traits (Debug, Clone, etc.)
#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let p1 = Point {x: 5, y: 6};
    let p2 = p1.clone();

    println!("{:?} {:?}", p1, p2);
    
}
