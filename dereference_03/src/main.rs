
use std::ops::Deref;
struct MyBox<T>(T);
impl<T> Deref for MyBox<T> {
    type Target = T; // syntax defines an associated type for the Deref trait to use
    // Without the Deref trait, the compiler can only dereference & references.
    // The deref method gives the compiler the ability to take a value of any type that
    // implements Deref and call the deref method to get a & reference that it knows how to dereference.
    // When we entered *y, behind the scenes Rust actually ran this code:
    // *(y.deref())
    fn deref(&self)-> &Self::Target {
        &self.0
    }
}
impl<T> MyBox<T> {
    fn new(x:T) -> MyBox<T>{
        MyBox(x)
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(5);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
