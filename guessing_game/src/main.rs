/*
* de esta manera de importan las librerias en rust
* a continuación utilizaremos la libreria std del core de rust
* https://doc.rust-lang.org/std/io/index.html
* TODO: Post sobre prelude, seria interesante hablar sobre esto.
* con esto acabo en el titulo "Processing a Guess"
*/
use std::io;

fn main() {
    println!("Adivine el numero!");
    println!("Por favor intente adivinar:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
    .expect("Fallo la lectura de la variable");
    println!("Usted ingreso: {}",guess);
}
