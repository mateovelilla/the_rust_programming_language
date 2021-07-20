
/**
 * Functions can return values to the code that calls them.
 * We don’t name return values, but we do declare their type after an arrow (->). 
 * In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
 * You can return early from a function by using the return keyword and specifying a value,
 * but most functions return the last expression implicitly.
 * Here’s an example of a function that returns a value:
 */
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}