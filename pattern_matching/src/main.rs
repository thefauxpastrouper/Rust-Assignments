// Pattern Matching: Use match to classify numbers as positive/negative/zero
fn pattern_matching(a: &i32) {
    match a {
        &x if x < 0 => println!("negative"),
        &0 => println!("zero"),
        &x if x > 0 => println!("positive"),
        _ => unreachable!()
    }
}

fn main() {
    pattern_matching(&2);
    pattern_matching(&0);
    pattern_matching(&-2);
}
