use std::slice;
// The function slice::from_raw_parts_mut is unsafe because it takes a raw pointer and
// must trust that this pointer is valid.
// The add method on raw pointers is also unsafe,
// because it must trust that the offset location is also a valid pointer.
// Therefore, we had to put an unsafe block around our calls to slice::from_raw_parts_mut and
// add so we could call them.
// By looking at the code and by adding the assertion that mid must be less than or equal to len,
// we can tell that all the raw pointers used within the unsafe block will be valid pointers to data within the slice.
// This is an acceptable and appropriate use of unsafe.

// Note that we don’t need to mark the resulting split_at_mut function as unsafe, 
// and we can call this function from safe Rust. 
// We’ve created a safe abstraction to the unsafe code with an implementation of the function that uses unsafe code
// in a safe way, because it creates only valid pointers from the data this function has access to.
fn split_at_mut(slice: &mut[i32], mid: usize) -> (&mut[i32], &mut[i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (

            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
fn main() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);
    println!("Tuples: {:?}, {:?}", left, right);
}
