use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::fmt;
use std::thread;

// Send + Sync: Understand and implement these traits
struct Carton<T> {
    ptr: NonNull<T>
}

impl<T> Carton<T> {
    pub fn new(value: T) -> Self {
        let boxed = Box::new(value);
        let ptr = NonNull::new(Box::into_raw(boxed)).unwrap();
        Self { ptr }
    }
}

impl<T> Deref for Carton<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> DerefMut for Carton<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T> Drop for Carton<T> {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.ptr.as_ptr());
        }
    }
}

unsafe impl<T: Send> Send for Carton<T> {}
unsafe impl<T: Sync> Sync for Carton<T> {}

impl<T: fmt::Display> fmt::Display for Carton<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<T: fmt::Debug> fmt::Debug for Carton<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}


fn main() {
    let carton = Carton::new(vec![1,2,4]);
    thread::spawn(move || {
        println!("From another thread: {:?}", *carton);
    }).join().unwrap();
}
