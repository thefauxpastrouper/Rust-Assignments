// Trait Inheritance: Create traits that inherit from other traits
trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

struct Graduate {
    student_name: String,
    university_name: String
}

impl Person for Graduate {
    fn name(&self) -> String {
        self.student_name.clone()
    } 
}

impl Student for Graduate {
    fn university(&self) -> String {
        self.university_name.clone()
    } 
}
fn main() {
    let person = Graduate {
        student_name: "Aditya".to_string(),
        university_name: "DAV".to_string()
    };

    println!("Student Name: {} and University Name: {}", person.student_name, person.university_name);
}
