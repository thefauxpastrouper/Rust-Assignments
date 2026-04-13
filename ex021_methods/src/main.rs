/// Methods: Implement methods for a Rectangle struct
struct Rectangle {
    length: u32,
    breadth: u32
} 

impl Rectangle {
    fn area(self: &Self) -> u32 {
        self.length * self.breadth
    }
}
fn main() {
    let square = Rectangle{
        length: 10,
        breadth: 10
};

    println!("{}", square.area());
}
