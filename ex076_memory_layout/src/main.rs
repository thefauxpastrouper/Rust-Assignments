// Memory Layout: Control struct memory layout with repr
use std::mem;

#[repr(C)]
struct CStruct {
    _a: u32,
    _b: u8,
    _c: u16
}

#[repr(packed)]
struct PackedStruct {
    _a: u32,
    _b: u8,
    _c: u16
}

#[repr(align(16))]
struct AlignedStruct {
    _data: u64
}

fn main() {
    println!("CStruct size: {}", mem::size_of::<CStruct>());
    println!("Packed Struct size: {}", mem::size_of::<PackedStruct>());
    println!("Aligned Struct size: {}", mem::size_of::<AlignedStruct>());
}
