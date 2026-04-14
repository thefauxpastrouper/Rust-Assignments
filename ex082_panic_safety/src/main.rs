struct Guard<'a, T> {
    vec: &'a mut Vec<T>,
    old_len: usize
}

impl <'a, T> Drop for Guard<'a, T> {
    fn drop(&mut self) {
        unsafe { self.vec.set_len(self.old_len); }
    }
}

fn push_all<T: Clone>(vec: &mut Vec<T>, slice: &[T]){
    vec.reserve(slice.len());

    let old_len = vec.len();
    let guard = Guard {
        vec,
        old_len
    };

    unsafe {
        let ptr = guard.vec.as_mut_ptr().add(guard.old_len);
        for (i, item) in slice.iter().enumerate() {
            ptr.add(i).write(item.clone());
        }
        guard.vec.set_len(guard.old_len + slice.len());
        std::mem::forget(guard);
    }
}

fn main() {
    let mut vec = vec![1,2];
    push_all(&mut vec, &[3,4,5,6]);
    println!("{:?}", vec);
}
