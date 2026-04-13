// Advanced Pattern Matching: Use pattern guards and @ bindings


fn main() {
    let point = (2,-2);
    match point {
        (x,y) if x == y => println!("Values are same!!!"),
        (x,y) if x + y == 0 => println!("Values are opposite!!!"),
        _ => println!("No special relationships")
    }
    
    #[derive(Debug)]
    enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write(String)
    }
    
    let msg = Message::Move {x: 1, y: 4};

    match msg {
        Message::Move {x: 0, y} => println!("On the y-axis at {}", y),
        Message::Move {x, y: 0} => println!("On the x-axis at {}", x),
        m @ Message::Move { .. } => println!("A move somewhere else: {:?}", m),
        _ => println!("Another message!!!")
    }
}
