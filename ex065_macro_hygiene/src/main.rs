macro_rules! swap {
    ($a: expr, $b: expr) => {
        {
            let temp = $a;
            $a = $b;
            $b = temp;
        }
    };
}

fn main() {
    let mut x = 1;
    let mut y = 23;
    let temp = 999;
    swap!(x, y);
    println!("x = {},y = {}, temp = {}", x, y, temp);
}

// In procedural macros, Using quote_spanned! with the input’s span helps maintain hygiene for generated identifiers.

