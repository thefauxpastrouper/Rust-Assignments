use std::ops::Add;

/// Generic Functions: Write a function that works with any type implementing Add
fn add<T: Add<Output = T>>(a: T, b: T)-> T {
    a + b
}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    } 
}

fn main() {
    let sum_i32 = add(2,4);
    println!("{}", sum_i32);

    let f_i64 = add(1.2, 4.3);
    println!("{}", f_i64);

    let point1 = Point {x: 2, y: 4};
    let point2 = Point {x: 5, y: 6};

    let added_point = add(point1, point2);
    println!("{:?}", added_point);
}
