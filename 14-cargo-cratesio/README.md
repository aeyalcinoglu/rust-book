# Release Profiles
- In Rust, *release profiles* are predefined and customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code.
- Cargo has two main profiles: `dev` and `release`.
- The `opt-level` setting controls the number of optimizations Rust will apply to the code, with a range of 0 to 3. Write these under `[profile.*]` sections.

# Documentation
- Rust has *documentation comments*, which will generate HTML documentation. It uses `///` and supports Markdown notation. Place them just before the item they're documenting.
- Running `cargo doc` runs `rustdoc` tool and puts the generated HTML documentation in the *target/doc* directory. Use `cargo doc --open` for convenience.
- Commonly used sections: Examples, Panics, Errors, Safety.
- Running `cargo test` will run the code examples in the documentation as tests.
- Another style of doc comment, `//!`, adds documentation to the item that containts it.

# Public API
- In cases where there are many nested modules, re-exporting the types at the top level with `pub use` can make a significant difference in the experience of people who use the crate.
- These will be mentioned as "Re-exports" in the main page of the documentation.

# Publishing
- Use `cargo login CARGO_REGISTRY_TOKEN`. It will be stored locally in *~/.cargo/credentials*.
- A description and license are required.
- Use `cargo publish` and be aware of [semver](https://semver.org/).
- Although you can’t remove previous versions of a crate, you can prevent any future projects from adding them as a new dependency. For this use `cargo yank --vers 1.0.1`.

# Cargo Workspaces
- A *workspace* is a set of packages that share the same *Cargo.lock* and output directory.
- To run the binary crate in a workspace, we can specify which package in the workspace we want to run by using the `-p` argument and the package name with `cargo run`.
- If one publishes the crates in the workspace to crates.io, each crate in the workspace will need to be published separately.

# Installing Binaries from Crates.io
- All binaries installed with `cargo install` are probably stored in *$HOME/.cargo/bin*.

# Custom Commands on Cargo
- Custom commands are listed via `cargo --list`.
