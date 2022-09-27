# General
- A package can contain multiple binary crates and optionally one library crate.
- The *module system* includes:
	- **Packages**: A Cargo feature that lets you build, test, and share crates
	- **Crates**: A tree of modules that produces a library or executable
	- **Modules** and **use**: Let you control the organization, scope, and privacy of paths
	- **Paths**: A way of naming an item, such as a struct, function, or module
- A crate is a binary or library. The *crate root* is a source file that the Rust compiler starts from and makes up the root module of your crate.
- A *package* is one or more crates that provide a set of functionality. A package contains a *Cargo.toml* file that describes how to build those crates.
- A package can have multiple binary crates by placing files in the *src/bin* directory.
- *Modules* let us organize code within a crate into groups for readability and easy reuse.
- *src/main.rs* and *src/lib.rs* are called crate roots, and either of them forms a module named crate at the root of the crate's module structure.

# Paths
- Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules.
- Using `super` at the start of the path is like starting a filesystem path with the `..` syntax.
- In contrast to struct, if we make an enum public, all of its variants are then public.
- Adding `use` and a path in a scope is similar to creating a symbolic link in the filesystem.
- The idiomatic way to bring a function into scope with `use` is to bring its parent module into scope. On the other hand, when bringing in structs, enums, and other items with `use`, it’s idiomatic to specify the full path.  

# Modules Cheat Sheet
- When compiling a crate, first `lib.rs` and `main.rs` are looked, as crate root.
- If "my_module" module is declared via `mod my_module;` in the crate root, the compiler further looks inline or at `src/garden.rs` and at `src/garden/mod.rs`.
- This works similar with declaring submodules. For going down, use `::` syntax.