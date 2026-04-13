// Loops: Print numbers 1-10 using for, while, and loop
fn main() {
    for i in 1..=10 {
        println!("{}", i);
    }

    let mut j = 1;
    while j <= 10 {
        println!("{}", j);
        j += 1;
    }

    let mut k = 1;
    loop {
        println!("{}", k);

        if k > 10 {
            break;
        }
        k += 1;
    }
}
