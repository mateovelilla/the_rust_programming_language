use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(40); // <== This statement validate if the key exists
    scores.entry(String::from("Blue")).or_insert(60);
    println!("{:?}", scores);
}
