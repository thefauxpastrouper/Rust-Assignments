// Associated Types: Create a trait with associated types
trait Container {
    type Item;
    fn add(&mut self, item: Self::Item);
    fn get(&self) -> Option<&Self::Item>;
}

struct StringContainer {
    value: Option<String>
} 

impl Container for StringContainer {
    type Item = String;
    fn add(&mut self, item: Self::Item) {
        self.value = Some(item);
    }

    fn get(&self) -> Option<&Self::Item> {
        self.value.as_ref()
    }
}
fn main() {
    let mut container = StringContainer {value: None};
    container.add("Hello".to_string());
    println!("{:?}", container.get());
}
