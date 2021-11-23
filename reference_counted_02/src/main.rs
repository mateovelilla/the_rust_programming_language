use std::rc::Rc; 
use crate::List::{Cons, Nil};
enum List {
    Cons(i32, Rc<List>),
    Nil,
}


fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil))))); // With RC we have share the same variable, RC only work with a single thread.
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}