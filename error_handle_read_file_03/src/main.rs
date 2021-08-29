use std::fs::File;
fn main() {
    let file = File::open("hello.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
