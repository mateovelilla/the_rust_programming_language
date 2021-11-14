struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPoint with data {}", self.data);
    }
}
fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    // You don't use drop if rust will deleted the variable
    // is like Rust try to delete the variable twice time
    c.drop();
    println!("CustomSmartPointer dropped before the end of main.");
}
