// Supertraits: Implement functionality requiring multiple traits
trait Shape {
    fn area(&self) -> f64;
}

trait Circle: Shape {
    fn radius(&self) -> f64 {
        (self.area() / std::f64::consts::PI).sqrt()
    }
}

struct MyCircle {
    radius: f64
}

impl Shape for MyCircle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Circle for MyCircle {
    fn radius(&self) -> f64 {
        self.radius
    }
}

fn describe_circle<C: Circle>(c: &C) {
    println!("Circle's area: {} and radius: {}", c.area(), c.radius());
}

fn main() {
    let circle = MyCircle { radius: 34.00 };
    // println!("My Circle's area: {} and radius: {}", circle.area(), circle.radius());
    describe_circle(&circle);
}
