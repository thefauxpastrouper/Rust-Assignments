use std::pin::Pin;
use std::marker::PhantomPinned;

#[derive(Debug)]
struct SelfRef {
    value: String,
    ptr: *const String,
    _pinned: PhantomPinned
}

impl SelfRef {
    fn new(value: String) -> Pin<Box<SelfRef>> {
        let mut s = Box::pin(SelfRef{
            value: value,
            ptr: std::ptr::null(),
            _pinned: PhantomPinned
        });
        unsafe{
            let mut_ref = Pin::as_mut(&mut s).get_unchecked_mut();
            mut_ref.ptr = &mut_ref.value as *const String;
        }
        s
    }

    fn get_value(&self) -> &str {
        &self.value
    }

    fn get_ptr_value(&self) -> &str {
        unsafe { &*self.ptr }
    }
}

fn main() {
    let selfref_object = SelfRef::new(String::from("Hello"));
    println!("Object: {:?}, Value: {}, Ptr pointing value: {}", selfref_object, selfref_object.get_value(), selfref_object.get_ptr_value())
}
