#[derive(Debug)]
struct Person<'a,'b> {
    name: &'a str,
    country: &'b str
}

fn main() {
    let name = String::from("Hello MOto");
    let country = "USA";

    let person = Person { name: &name, country };
    println!("{:?}", person);
}
