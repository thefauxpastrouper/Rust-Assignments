use std::marker::PhantomData;

// PhantomData: Use PhantomData for type safety in generic structs
struct UserId;
struct ProductId;

struct Id<T> {
    value: u64,
    _marker: PhantomData<T> // Mark association with T
}

fn process_user(id: Id<UserId>) {
    println!("User: {}", id.value);
}

#[derive(Debug)]
struct Slice<'a, T> {
    _ptr: *const T,
    _len: usize,
    _marker: PhantomData<&'a T>
}

impl <'a, T> Slice<'a, T> {
    unsafe fn new(slice: &'a [T]) -> Self {
        Self {
            _ptr: slice.as_ptr(),
            _len: slice.len(),
            _marker: PhantomData
        }
    }
}

fn main() {
    let user = Id::<UserId> {value: 1, _marker: PhantomData};
    let _product = Id::<ProductId> {value: 1, _marker: PhantomData};

    process_user(user);
    // process_user(product); // will cause error

    let data = vec![1, 2, 3, 4];
    let slice = unsafe { Slice::new(&data) };
    println!("{:?}", slice);
}
