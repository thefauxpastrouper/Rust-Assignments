// Unsafe Functions: Mark and call unsafe functions appropriately
use std::slice;

fn split_at_mut(sliced: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = sliced.len();
    let ptr = sliced.as_mut_ptr();

    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}

fn main() {
    let mut arr = [1,3,4,56,6];
    let mid = 3;

    let (result1, result2) = split_at_mut(&mut arr, mid);
    println!("Result1: {:?}, Result2: {:?}", result1, result2);
}
