@ Bindings

// The at operator (@) lets us create a variable that holds a value at the same time
// we’re testing that value to see whether it matches a pattern. 
// shows an example where we want to test that a Message::Hello id field is within the range 3..=7. 
// But we also want to bind the value to the variable id_variable so we can use it in the code associated 
// with the arm. We could name this variable id, the same as the field, but for this example
// we’ll use a different name.
fn main() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
