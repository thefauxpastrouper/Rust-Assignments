// Never Type: Use ! type in functions that never return
fn loop_forever()-> ! {
    loop {
        println!("Hello !!!");
    }
}

fn main() {
    loop_forever();
    println!("Hello, world!");
}
