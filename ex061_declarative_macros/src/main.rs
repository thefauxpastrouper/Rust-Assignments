/// Declarative Macros: Create a vec!-like macro
macro_rules! my_vec {
    () => {{
        Vec::new()    
    }};

    ($($item:expr),+ $(,)?) => {{
        let mut temp_vec = Vec::new();
        $(temp_vec.push($item);)+
        temp_vec
    }};
}


fn main() {
    let v1: Vec<i32> = my_vec![1,2,3,4];
    let v2: Vec<i32> = my_vec![];
    let v3: Vec<char> = my_vec!['a', 'b', 'c'];

    println!("{:?}, {:?}, {:?}", v1, v2, v3);
}
