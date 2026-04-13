// Lifetime Subtyping: Understand and apply lifetime variance
struct Container<'a> {
    _data: &'a str
}

fn take_container(_c: Container<'_>) {}

fn main() {
    let long = "static str";
    let _short = String::from("Hello short str here");

    let container = Container { _data: long }; // Container<'static>
    take_container(container); // Container can be weakened to shorter lifetime

    println!("Hello, world!");
}
