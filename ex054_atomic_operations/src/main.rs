use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::sync::Arc;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 1..=10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 1..=10 {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter: {}", counter.load(Ordering::Relaxed));
}
