enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};
// The error is why a variables can't share twice time
fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}