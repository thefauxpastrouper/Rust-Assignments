enum MyOption<T> {
    Some(T),
    None
}

fn check_positive_or_zero<T>(a: MyOption<T>)-> String {
    match a {
    MyOption::Some(_) => "positive".to_string(),
    MyOption::None => "zero".to_string()
    }
}
fn main() {
    let a: MyOption<i8> = MyOption::Some(23);
    let b: MyOption<i8> = MyOption::None;

    println!("{}", check_positive_or_zero(a));
    println!("{}", check_positive_or_zero(b));
}
