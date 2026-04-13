use std::sync::{Arc, Mutex};
use std::thread;

/// Shared State: Implement thread-safe counter with Arc<Mutex<T>>
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 1..=10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Num value after modification: {}", *counter.lock().unwrap());

    println!("Hello, world!");
}
