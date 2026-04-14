// Inline Assembly: Use inline assembly for performance-critical code
use std::arch::asm;

fn read_tsc() -> u64 {
    let tsc: u64;
    unsafe {
        asm!(
            "rdtsc",
            "shl rdx, 32",
            "or rax, rdx",
            out("rax") tsc,
            out("rdx") _
        );
    }
    tsc
}

fn count_set_bits(value: u32) -> u32 {
    let count: u32;
    unsafe {
        asm!(
            "popcnt {0:e}, {1:e}",
            out(reg) count,
            in(reg) value,
            options(nostack, preserves_flags)
        );
    }
    count
}

fn main() {
    let tsc = read_tsc();
    println!("TSC Value: {}", tsc);
    let value: u32 = 3;
    let count = count_set_bits(value);
    println!("Count: {}", count);
}
