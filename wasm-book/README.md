Reading notes for the book [Rust 🦀 and WebAssembly 🕸](https://rustwasm.github.io/docs/book/)

# Why
- Low-level control with high-level ergonomics: JS is not reliable in performance, there are unstructured pauses. It's a bit like a hash function, JIT's happy path.
- Rust lacks a runtime, enabling small `.wasm`, no extras like GC.

# Background

- The `.wat` text format, uses S-expressions. Like Scheme.
- The `.wasm` binary format is lower-level and intended for wasm VM. Like ELF.
- A wasm module has access to a single "linear memory", a flat array of bytes. Can grow by multiple of 64K, cannot be shrunk.

# Hello, World!
- The difference between `cargo generate --git` and `git clone` is generally minimal, especially in this case of `rustwasm/wasm-pack-template`.
- After every change run `wasm-pack build`.
- Doing the exercise at the end of the chapter 4.2 by passing a `String` to `alert` instead of referencing the `format!` causes an extra `try... finally` statement where `wasm.__wbindgen_free` is used.

# Interfacing Rust and JavaScript
- JavaScript's garbage-collected heap — where `Object`s, `Array`s, and DOM nodes are allocated — is distinct from WebAssembly's linear memory space, where the Rust values live.
- WebAssembly has no direct access to the garbage-collected heap. JavaScript, on the other hand, can read and write to the WebAssembly linear memory space.
- `wasm_bindgen` defines a common understanding of how to work with compound structures across this boundary. It can be thought as a tool for implementing the interface design you choose.
- A general rule of thumb: Implement large, long-lived data structures as Rust types that live in the WebAssembly linear memory, and expose them to Javascript as opaque handles.

# Implementation
- The `window.requestAnimationFrame()` method tells the browser that you wish to perform an animation and requests that the browser calls a specified function to update an animation before the next repaint.
- Generating (and allocating) a `String` in Rust and then having `wasm-bindgen` convert it to a valid JavaScript string makes unnecessary copies of the universe's cells.
- We can directly access WebAseembly's linear memory via `memory`, which is defined in the raw wasm module `wasm_game_of_life_bg`.

# Testing
- One can have another implementation of a struct without the `#[wasm-bindgen]` attribute, in order for the usage in Rust without exposing it to JavaScript.
- Rust-generated WebAssembly functions cannot return borrowed references.
- Use `#[wasm_bindgen_test]` attribute to be able to use `wasm-pack test`.
- Use `wasm-pack test --firefox --headless` to run the tests.

# Debugging
- If you don't have debug symbols enabled, then the `"name"` custom section won't be present in the compiled `.wasm` binary.
- To enable debug symbols in "release" build, put `debug = true` in the `[profile.release]` section in `Cargo.toml`.
- Use `web_sys` crate to get access to the `console` logging functions.
- The `console_error_panic_hook` crate logs unexpected panics to the developer console via `console.error`. 
