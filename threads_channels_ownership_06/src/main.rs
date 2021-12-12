use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // Here, we try to print val after we’ve sent it down the channel via tx.send. 
        // Allowing this would be a bad idea: once the value has been sent to another thread, 
        // that thread could modify or drop it before we try to use the value again. 
        // Potentially, the other thread’s modifications could cause errors or unexpected results due to inconsistent or nonexistent data.
        // However, Rust gives us an error if we try to compile the code in Listing 
        println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}