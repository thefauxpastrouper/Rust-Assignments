use std::sync::mpsc;
use std::thread;

/// Message Passing: Use channels for thread communication
fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        println!("Hi from the thread!!!");
        let msg = String::from("Message");
        tx.send(msg).unwrap();
    });

    let m = rx.recv().unwrap();
    println!("Message Received: {}", m);

    println!("Hello, world!");
}
