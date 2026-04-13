use std::fmt::Display;

trait Processor {
    fn process<T: Display>(&self, value: T) -> String;
}

struct MyProcessor ;

impl Processor for MyProcessor {
    fn process<T:Display>(&self, value: T) -> String {
        format!("Formatted: {}", value)
    }
}

fn get_processor<P>(processor: P) -> impl for<'a> Fn(&'a str) -> String 
where 
    P: Processor
{
    move |value| processor.process(value)
}

fn main() {

    let myprocessor = MyProcessor;

    let proc_closure = get_processor(myprocessor);

    let input = "Hello String";
    let input2 = String::from("Dynamic String");

    let result = proc_closure(input);
    let result2 = proc_closure(&input2);

    println!("Result: {}, Result2: {}", result, result2);
}
