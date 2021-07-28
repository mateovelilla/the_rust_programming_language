// mutable references have one big restriction: you can have only one mutable reference to a
// particular piece of data in a particular scope. This code will fail:
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
