// Until now, we’ve not talked about global variables,
// which Rust does support but can be problematic with Rust’s ownership rules.
// If two threads are accessing the same mutable global variable,
// it can cause a data race.
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}