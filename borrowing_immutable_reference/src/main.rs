// Note that a reference’s scope starts from where it is introduced and continues
// through the last time that reference is used. For instance,
// this code will compile because the last usage of the immutable
// references occurs before the mutable reference is introduced:

fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// The scopes of the immutable references r1 and r2 end after the println!
// where they are last used, which is before the mutable reference r3 is created.
// These scopes don’t overlap, so this code is allowed.
// Even though borrowing errors may be frustrating at times,
// remember that it’s the Rust compiler pointing out a potential bug early
// (at compile time rather than at runtime) and showing you exactly where the problem is.
// Then you don’t have to track down why your data isn’t what you thought it was.
