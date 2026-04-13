// Iterators Basic: Use iterator adaptors (map, filter, collect)

fn main() {
    let v = vec![1, 2, 3, 4, 4, 4, 4, 5];
    let p: Vec<i32> = v.iter().map(|x| x * 2).collect();
    let q: Vec<i32> = v.iter().filter(|x| *x % 2 == 0).cloned().collect();
    println!("Mapped vector: {:?} Filtered Vector: {:?}", p, q);
}
