fn first_word<'a>(input: &'a str)-> &'a str{
    let bytes = input.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &input[0..i]
        }
    }
    &input[..]
}

fn copy_borrow<'a, 'b>(x: &'a i32, target: &'b mut &'a i32) where 'a: 'b {
    *target = x;
}

fn main() {
    let input = "Hekko World";
    println!("{}", first_word(input));

    let value = 42;
    let mut slot: &i32 = &0;

    {
        let x = &value;
        copy_borrow(x, &mut slot);
    }
    
    println!("{}", slot);
}
