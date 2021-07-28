// Whew! We also cannot have a mutable reference while we have an immutable one. 
// Users of an immutable reference don’t expect the values to suddenly change out from under them!
// However, multiple immutable references are okay because no one who is just reading the data has 
// the ability to affect anyone else’s reading of the data.
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}
