use std::sync::mpsc;
use std::thread;
fn main() {
    let (tx, rx) = mpsc::channel(); // multiple producer, single consumer (tx => transmitter, rx => receiver)
    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap(); // the send method return => Result<T, E> and we use the unwrap method to panic in case of an error
    });
    // the recv method return => Result<T, E> and we use the unwrap method to panic in case of an error
    // The recv block the main thread
    // the try_recv method don't block the main thread
    let received = rx.recv().unwrap();
    println!("Got: {}", received);  

}