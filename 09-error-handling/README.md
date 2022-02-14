- Rust groups errors into two major categories: *recoverable* and *unrecoverable* errors.
- Rust doesn't have exceptions. Instead, it has the type `Result<T, E>` for recoverable errors and the `panic!` macro that stops execution when the program encounters and unrecoverable error.

# Panic
- When the `panic!` macro executes, your program will print a failure message, unwind and clean up the stack, and then quit. In order to skip unwinding and abort upon a panic, add ``panic = `abort` `` to the appropriate `[profile]` sections of the *Cargo.toml* file, e.g. `[profile.release]`.
- We can set the `RUST_BACKTRACE` environment variable to get a backtrace of exactly what happened to cause the error. A *backtrace* is a list of all the functions that have been called to get to this point.
```
RUST_BACKTRACE=1 cargo run
```

# Result
- The `Result` enum is defined as having two variants, `Ok` and `Err`, as follows:
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
- Calling `unwrap` on a `Result` will do `panic!` or return the value inside the `Ok`.
- Calling `expect` works like `unwrap` but lets us also choose the `panic!` error message.
- We’re only allowed to use the `?` operator in a function that returns `Result`, `Option`, or another type that implements `FromResidual`.
- The types that `main` may return are those that implement the `std::process::Termination` trait.
