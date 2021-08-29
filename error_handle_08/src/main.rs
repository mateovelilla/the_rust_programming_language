use std::fs::File;
use std::io;
use std::io::Read;
fn read_username_from_file() -> Result<String, io::Error> {
    let file = File::open("hello.txt");
    let mut file = match file {
        Ok(fileToRead) => fileToRead,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
fn main() {
    read_username_from_file();
}
