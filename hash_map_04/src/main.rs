use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Yellow"), 30);
    let team_core = String::from("Blue");
    let score = scores.get(&team_core);
    println!("{:?}", score);
}
