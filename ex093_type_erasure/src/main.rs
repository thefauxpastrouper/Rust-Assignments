trait Speaker {
    fn speak(&self);
}

struct Dog;
impl Speaker for Dog {
    fn speak(&self) { println!("Woff!!")}
}

struct Human;
impl Speaker for Human {
    fn speak(&self) { println!("Hello !!") }
}

fn process(speaker: &dyn Speaker) {
    speaker.speak();
}

fn main() {
    let dog = Dog;
    let human = Human;

    process(&dog);
    process(&human);
}
