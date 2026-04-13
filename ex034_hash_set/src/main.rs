// HashSet: Remove duplicates from a vector
use std::collections::HashSet;

fn remove_duplicates(vec: Vec<i32>) -> Vec<i32> {
    let mut set = HashSet::new();
    
    vec.into_iter()
        .filter(|item| set.insert(*item))
        .collect()
} 

fn main() {
    let v = vec![1,2,3,3,4];
    let unique = remove_duplicates(v);
    println!("{:?}", unique);        
}
