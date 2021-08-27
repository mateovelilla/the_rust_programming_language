use std::collections::HashMap;
fn main() {
    let teams = vec![String::from("Blue"),String::from("Yellow")];
    let initial_score = vec![10, 50];
    let scores: HashMap<_, _> = 
        teams.into_iter()
        .zip(initial_score.into_iter())
        .collect();
    println!("{:?}", scores);
}
