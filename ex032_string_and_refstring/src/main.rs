// String vs &str: Convert between String and string slices appropriately
// String has larger metadata (pointer, length, capacity).
// &str is a fat pointer (pointer + length)
fn main() {
    let slice = "Hello";
    let owned = slice.to_string();
    println!("size of Owned: {:?} bytes", std::mem::size_of_val(&owned));

    let slice = "Hello";
    let owned = slice.to_owned();
    println!("Size of Owned: {:?} bytes", std::mem::size_of_val(&owned));

    let string_s = String::from("Hello");
    let str = string_s.as_str();
    println!("Size of String slice: {:?} expected 16 bytes", std::mem::size_of_val(&str));

    let string_s = String::from("Hello");
    let str = &string_s;
    println!("Size of String slice: {:?} expected 16 bytes", std::mem::size_of_val(&str));
}
