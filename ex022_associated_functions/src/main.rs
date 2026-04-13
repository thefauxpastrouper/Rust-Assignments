#[derive(Debug)]
struct Rectangle {
    length: u32,
    breadth: u32
}

/// Associated Functions: Create constructor methods using impl
impl Rectangle {
    fn new(length: u32, breadth: u32)-> Self {
        Self{length, breadth}
    }

    fn square(side: u32) -> Self {
        Self {length: side, breadth: side}
    }
}

fn main() {
    let rect = Rectangle::new(4,5);
    println!("Rectangle: {:?}", rect);
    let sq = Rectangle::square(4);
    println!("Square: {:?}", sq);
}
