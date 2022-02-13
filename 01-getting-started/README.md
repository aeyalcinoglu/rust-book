# Format, Compile and Run
- `rustfmt file.rs`
- `rustc file.rs`
- `./file`

# Creating a Project with Cargo
- `cargo new project_name`
- Use `cargo new --vcs=git` for overriding Git repository initialization.
## Building and Running a Cargo Project
- In the project directory, run `cargo build` (use `--release` for optimizations)
- Then run `./target/debug/project_name`
- Or alternatively, just run `cargo run`
- The easiest way to run rustfmt against a project is with `cargo fmt`
- Also, use `cargo check` to check if it compiles without producing an executable.


# Other
- The `main` function is special: it is always the first code that runs in every executable Rust program.
- Using a `!` means that you’re calling a macro instead of a normal function.
- Cargo is Rust’s build system and package manager.
- In Rust, packages of code are referred to as **crates**.
- Use [rust-lang/rust.vim](https://github.com/rust-lang/rust.vim) as a Vim plugin.
