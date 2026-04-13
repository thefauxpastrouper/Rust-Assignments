use crossbeam::thread;

fn main() {
    let v = vec![1,2,3,4,5,6];
    
    thread::scope(|s| {
        s.spawn(|_| {
           for num in &v {
                println!("Scoped thread: {}", num);
            } 
        });
    }).unwrap();
}
