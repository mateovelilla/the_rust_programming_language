fn main() {
    // The expression in the if block evaluates to an integer,
    // and the expression in the else block evaluates to a string.
    // This won’t work because variables must have a single type.
    // Rust needs to know at compile time what type the number variable is, definitively,
    // so it can verify at compile time that its type is valid everywhere we use number.
    // Rust wouldn’t be able to do that if the type of number was only determined at runtime;
    // the compiler would be more complex and would make fewer guarantees about the code if
    // it had to keep track of multiple hypothetical types for any variable.
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {}", number);
}