// Memory Safety: Audit unsafe code for memory safety violations

fn main() {
    let arr = [1,2,3];
    unsafe { 
        *arr.as_ptr().offset(10);
    };
}
