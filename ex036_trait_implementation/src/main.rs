use std::fmt;

/// Trait Implementation: Implement Display trait for a custom type
#[derive(Debug)]
struct Point {
    x: u32,
    y: u32
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let point = Point {x: 1, y: 3};
    println!("Point is {}", point);
}
