// Arrays: Create an array of 5 integers and print all elements
fn main() {
    let arr = [1,2,4];
    
    println!("Array: {:?}", arr);

    for num in arr.iter() {
        println!("{}", num);
    }
}
