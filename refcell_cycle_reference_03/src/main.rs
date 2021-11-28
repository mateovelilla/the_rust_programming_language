use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
fn main(){
    let a = Rc::new(
        Cons(
            5, 
            RefCell::new(
                Rc::new(Nil)
            )
        )
    );

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(
        Cons(
            10, 
            RefCell::new(
                Rc::clone(&a)
            )
        )
    );

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // println!("a next item = {:?}", a.tail()); <== it will overflow the stack
}
// We create an Rc<List> instance holding a List value in the variable a with an initial list of 5, Nil.
// We then create an Rc<List> instance holding another List value in the variable b that contains the value 10 and points to the list in a.
// We modify a so it points to b instead of Nil, creating a cycle.
// We do that by using the tail method to get a reference to the RefCell<Rc<List>> in a,
// which we put in the variable link. Then we use the borrow_mut method on the RefCell<Rc<List>> to change the value inside from an Rc<List> that holds a Nil value to the Rc<List> in b.
// If you uncomment the last println! and run the program, Rust will try to print this cycle with a pointing to b pointing to a and so forth until it overflows the stack.

// In this case, right after we create the reference cycle, the program ends.
// The consequences of this cycle aren’t very dire. However,
// if a more complex program allocated lots of memory in a cycle and held onto it for a long time,
// the program would use more memory than it needed and might overwhelm the system, causing it to run out of available memory.