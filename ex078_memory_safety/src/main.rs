// Memory Safety: Audit unsafe code for memory safety violations
pub unsafe trait IntoBytes {
    fn as_bytes(&self) -> &[u8] {
        let len = std::mem::size_of_val(self);
        let ptr = self as *const Self as *const u8;

        unsafe { std::slice::from_raw_parts(ptr, len) }
    }
}

unsafe impl IntoBytes for u32 {}

fn main() {
    let value: u32 = 0x12345678;
    let bytes = value.as_bytes();

    println!("u32 {:x} as bytes {:x?}", value, bytes);
}
