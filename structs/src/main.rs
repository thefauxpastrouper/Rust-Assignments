// Structs Basic: Define a Person struct with name and age fields
#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

fn main() {
    let person1 = Person {name : String::from("Aditya"), age: 23};
    println!("{:?}", person1); 
}
