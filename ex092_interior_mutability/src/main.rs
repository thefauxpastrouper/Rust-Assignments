// Interior Mutability: Use RefCell, Cell, and UnsafeCell
use std::cell::Cell;
use std::cell::RefCell;
use std::cell::UnsafeCell;

fn main() {
    let data = Cell::new(5);
    data.set(10);
    println!("Data: {}", data.get());

    let data = RefCell::new(vec![1,2,3,4]);
    {
        let mut borrowed = data.borrow_mut();
        borrowed.push(5);
    }
    println!("Data: {:?}", data.borrow());

    let data = UnsafeCell::new(5);
    unsafe {
        *data.get() = 10;
        println!("Data: {}", *data.get())
    }
    
}
