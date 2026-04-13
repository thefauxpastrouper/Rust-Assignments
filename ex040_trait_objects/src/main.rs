// Trait Objects: Create a vector of trait objects
trait Drawable {
    fn draw(&self);
}

struct Square;
impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square!!!");
    }
}

struct Circle;
impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a Circle!!!");
    }
}

fn main() {
    // create a vector of trait objects
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Square),
        Box::new(Circle)
    ];    

    for shape in &shapes {
        shape.draw();
    }
}
