/**
 * is a block that, in this case, evaluates to 4. 
 * That value gets bound to y as part of the let statement. 
 * Note the x + 1 line without a semicolon at the end, 
 * which is unlike most of the lines you’ve seen so far. 
 * Expressions do not include ending semicolons. 
 * If you add a semicolon to the end of an expression, you turn it into a statement,
 * which will then not return a value. 
 * Keep this in mind as you explore function return values and expressions next.
 */
fn main() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}