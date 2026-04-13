// Raw Pointers: Convert between references and raw pointers
// The conversions like &T -> *const T and &mut T -> *mut T are considered to be always valid
fn main() {
    let value: i32 = 42;
    let reference: &i32 = &value;
    
    // Explicit cast
    let raw_const: *const i32 = &value as *const i32;
    let raw_const_safe = std::ptr::from_ref(reference);
    println!("Raw pointer constants: {:?}  and {:?}", raw_const, raw_const_safe);

    // Implicit conversion
    let mut y: i32 = 10;
    let raw_implicit: *const i32 = &mut y;
    println!("Raw implicit: {:?}", raw_implicit);

    let mut x  = 5;
    let raw: *const i32 = &x as *const i32;
    unsafe {
        let safe_ref: &i32 = &*raw; // dereference and reborrow
        let mut_ref: &mut i32 = &mut *(&mut x as *mut i32);
        println!("Safe const ref: {:?}, Mutable ref: {:?}", safe_ref, mut_ref);

    }
    
}
