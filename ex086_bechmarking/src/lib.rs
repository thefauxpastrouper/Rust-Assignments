pub fn print_fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => print_fibonacci(n - 1) + print_fibonacci(n - 2),
    }
}
