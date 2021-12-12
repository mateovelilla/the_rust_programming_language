# 🦀 The Rust Programming Language  
Those are personal exercises take in of the [public book](https://doc.rust-lang.org/book/) to rustaceans beginners,
I'm very excited to learn all the weird features about this language programing so, Go ahead!
# the_rust_programming_language
The Rust Programming Language Book
https://doc.rust-lang.org/book/ch16-03-shared-state.html

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

🦀 [Implement dereference](https://doc.rust-lang.org/book/ch15-02-deref.html#treating-a-type-like-a-reference-by-implementing-the-deref-trait)

🦀 [the Reference Counted Smart Pointer - multiple reference to a value](https://doc.rust-lang.org/book/ch15-04-rc.html)

🦀 [Function Cons](https://docs.rs/im/5.0.0/im/list/fn.cons.html)

🦀 [the reference counted in the doc](https://doc.rust-lang.org/std/rc/struct.Rc.html)

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
- `cargo run -p adder`: -p To run a package in specific.
- `cargo test -p add-one`: To run the test of specific packages
- `cargo install ripgrep`: To install a binary.
- `cargo --list`: To list all subcommands of cargo

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
- When use Boxes in Rust:
    - When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size.
    - When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so.
    - When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type.

### TIPS:

- `Cargo has two main profiles: the dev profile Cargo uses when you run cargo build and the release profile Cargo uses when you run cargo build --release. The dev profile is defined with good defaults for development, and the release profile has good defaults for release builds.` 
- `If you publish the crates in the workspace to crates.io, each crate in the workspace will need to be published separately. The cargo publish command does not have an --all flag or a -p flag, so you must change to each crate’s directory and run cargo publish on each crate in the workspace to publish the crates`

### TODO:

- [ ] Find how extends cargo
