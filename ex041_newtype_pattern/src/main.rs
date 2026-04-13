use regex::Regex;

// Newtype Pattern: Create a wrapper type with validation
#[derive(Debug, Clone, PartialEq)]
struct Email(String);

#[derive(Debug)]
struct EmailError(String);

impl Email {
    pub fn new(s: &str) -> Result<Self, EmailError> {
        static EMAIL_REGEX: &str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";
        let re = Regex::new(EMAIL_REGEX).unwrap();

        if re.is_match(s) {
            Ok(Email(s.to_string()))
        }
        else {
            Err(EmailError(s.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl EmailError {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}


fn main() {
    let email = Email::new("aditya@gmail.com");
    match email {
        Ok(e) => println!("Valid email: {}", e.as_str()),
        Err(s) => println!("Error creating Email!!!! Given email: {}", s.as_str())
    }

    let email = Email::new("adityagmail.com");
    match email {
        Ok(e) => println!("Valid email: {}", e.as_str()),
        Err(s) => println!("Error creating Email!!!! Given email: {}", s.as_str())
    }
}
