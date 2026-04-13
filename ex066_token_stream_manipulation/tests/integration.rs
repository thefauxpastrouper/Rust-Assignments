use ex066_token_stream_manipulation::trace;

#[trace]
fn example() {
    println!("Test Function");
}

#[test]
fn run_with_macro() {
    example();
}
