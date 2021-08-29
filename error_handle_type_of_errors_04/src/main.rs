use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let file = File::open("hello.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fileCreated) => fileCreated,
                Err(errorToCreate) => panic!("Problem creating the file: {:?}", errorToCreate),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };
}
