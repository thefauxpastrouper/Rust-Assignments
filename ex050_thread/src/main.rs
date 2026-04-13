use std::thread;
use std::time::Duration;

/// Threads: Spawn threads and wait for completion
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi from spawned thread!!!!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi from the main thread!!!");
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
    println!("Hello, world!");
    
}
