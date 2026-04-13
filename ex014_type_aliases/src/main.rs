// Type Aliases: Create type aliases for complex types
use std::collections::HashMap;

type UserScores = HashMap<String, Vec<i32>>;

fn main() {
    let _scores: UserScores = HashMap::new();
}
