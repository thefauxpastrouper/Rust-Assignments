/// Lifetimes Basic: Add lifetime annotations to struct with references
struct MyStruct<'a> {
    data: &'a str
}

fn main() {
    let data = String::from("Data");
    let mystruct = MyStruct {data: &data};
    println!("Data: {}", mystruct.data);
}
