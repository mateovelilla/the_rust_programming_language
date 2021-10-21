# 🦀 The Rust Programming Language  
Those are personal exercises take in of the [public book](https://doc.rust-lang.org/book/) to rustaceans beginners,
I'm very excited to learn all the weird features about this language programing so, Go ahead!
# the_rust_programming_language
The Rust Programming Language Book
https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html
## Reference links:
🦀 https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization

🦀 https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop

🦀 https://doc.rust-lang.org/book/appendix-03-derivable-traits.html

🦀 https://doc.rust-lang.org/cargo/reference/manifest.html

🦀 https://doc.rust-lang.org/std/collections/index.html

🦀 https://doc.rust-lang.org/book/ch13-02-iterators.html

🦀 https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#validating-references-with-lifetimes

🦀 https://en.wikipedia.org/wiki/SipHash

🦀 https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types

🦀 https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters

🦀 https://stackoverflow.com/questions/14189604/what-is-monomorphisation-with-context-to-c#14198060

🦀  https://en.wikipedia.org/wiki/Monomorphization

🦀  https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types

🦀 https://doc.rust-lang.org/unstable-book/library-features/test.html

🦀 [Importants rules](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision)

🦀 https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests

🦀 https://doc.rust-lang.org/rust-by-example/fn/closures.html

🦀 https://doc.rust-lang.org/std/iter/trait.Iterator.html

---
### IMPORTANT COMMANDS
-   `cargo test -- --test-threads=1`: create multiple threads to run the test.
- `cargo test -- --show-output`: When Rust runs the tests only shows the outputs when the test failed, this flag enables the option to show the outputs if the tests failed or success.
- `cargo test one_hundred`: the way to run only the test, `one_hundred` is the name of the test to execute.
- `cargo test add`: this command executes only tests that contains `add` in the name of the function test.
- `cargo test -- --ignored`: Runs only the tests that were ignored.
- `cargo test --test integration_test`: Runs only the integration test in the `tests` folder.
- `CASE_INSENSITIVE=1 cargo run to poem.txt`: Run Rust with environment variables for example: `CASE_INSENSITIVE=1`
- `cargo run > output.txt`: To redirect the standard output to a file.

### ADVICES:
- `Closures and iterators are Rust features inspired by functional programming language ideas. They contribute to Rust’s capability to clearly express high-level ideas at low-level performance. The implementations of closures and iterators are such that runtime performance is not affected. This is part of Rust’s goal to strive to provide zero-cost abstractions.`
