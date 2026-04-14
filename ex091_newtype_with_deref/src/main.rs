use std::ops::Deref;

struct Email(String);

impl Deref for Email {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    } 
}

fn main() {
    let email = Email("aditya@gmail.com".to_string());
    println!("Email Length: {}", email.len());
}
