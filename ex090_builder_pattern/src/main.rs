// Builder Pattern: Implement type-safe builder pattern
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Debug)]
struct User {
    _name: String,
    _email: String,
    
    #[builder(default)]
    _age: u8,
    #[builder(default = 1)]
    _score: u8,
}

fn main() {
    let user = User::builder()
                ._name("Aditya".to_string())
                ._email("aditya@gmail.com".to_string())
                .build();
    println!("Hello, world! User: {:?}", user);
}
