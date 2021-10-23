# 🦀 The Rust Programming Language  
Those are personal exercises take in of the [public book](https://doc.rust-lang.org/book/) to rustaceans beginners,
I'm very excited to learn all the weird features about this language programing so, Go ahead!
# the_rust_programming_language
The Rust Programming Language Book
https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html#creating-the-second-package-in-the-workspace
## Reference links:
🦀 [Resource acquisition is initialization](https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization)

🦀 [Drop Destructor](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop)

🦀 [Derive](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)

🦀 [Manifest in cargo](https://doc.rust-lang.org/cargo/reference/manifest.html)

🦀 [Collections](https://doc.rust-lang.org/std/collections/index.html)

🦀 [Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)

🦀 [LifeTime](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#validating-references-with-lifetimes)

🦀 [SipHash](https://en.wikipedia.org/wiki/SipHash)

🦀 [Build object with differents types](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types)

🦀 [Traits like paramas](https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters)

🦀 [Monomorphization](https://en.wikipedia.org/wiki/Monomorphization)

🦀 [Importants rules](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision)

🦀 [Tests](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests)

🦀 [Clouse](https://doc.rust-lang.org/rust-by-example/fn/closures.html)

🦀 [Iterators in deep](https://doc.rust-lang.org/std/iter/trait.Iterator.html)

🦀 [Cargo](https://doc.rust-lang.org/cargo/)

🦀 [SPDX License List](https://spdx.org/licenses/)

🦀 [Semantic Versioning rules](https://semver.org/)

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
- `cargo doc`: To generate documentation storage in the folder `target/doc` in HTML format.
- `cargo doc --open`: To build the HTML for your current crate's documentation.
- `cargo login abcdefghijklmnopqrstuvwxyz012345`: To login in crates
- `cargo publish`: To publish a package in [crates.io](https://crates.io/)
- `cargo yank --vers 1.0.1`: To delete version of any package of your crates account

### ADVICES:

- `Closures and iterators are Rust features inspired by functional programming language ideas. They contribute to Rust’s capability to clearly express high-level ideas at low-level performance. The implementations of closures and iterators are such that runtime performance is not affected. This is part of Rust’s goal to strive to provide zero-cost abstractions.`
- `To generate documentation in Rust you can put 3 slash and it'll interpreter in Markdown format, for example:`
    ```
        /// ### Heading
    ```
- `to add documentation to the item that contains the comments rather than adding documentation to the items following the comments you can use //!`
    ```
        //! # My Crate
        //!
        //! `my_crate` is a collection of utilities to make performing certain
        //! calculations more convenient.

        /// Adds one to the number given.
    ```
### TIPS:

- `Cargo has two main profiles: the dev profile Cargo uses when you run cargo build and the release profile Cargo uses when you run cargo build --release. The dev profile is defined with good defaults for development, and the release profile has good defaults for release builds.` 
