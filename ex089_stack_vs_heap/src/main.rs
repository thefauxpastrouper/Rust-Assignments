// Stack vs Heap: Optimize memory allocation patterns
use std::fmt;

#[derive(Debug)]
struct SmallData([u8; 16]);

fn process() -> SmallData {
    let data = SmallData([0u8; 16]);
    data
}

fn process_heap_data() -> Vec<u8>{
    let good = vec![0u8; 10_000_000];
    good
}

struct CompactVec<T>(Vec<T>);
impl<T: fmt::Debug> fmt::Debug for CompactVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
         .entries(self.0.iter().take(5)) // Show first 5 elements
         .finish()?;
        if self.0.len() > 5 {
            write!(f, " ... +{} more", self.0.len() - 5)
        } else {
            Ok(())
        }
    }
}


fn main() {
    let data = process();
    println!("Stack allocated Data(For small data): {:?}", data);
    let good = process_heap_data();
    println!("Heap Allocated Data(For Large data): {:?}", CompactVec(good));
}
