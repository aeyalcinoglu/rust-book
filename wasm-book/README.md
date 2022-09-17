Reading notes for the book [Rust 🦀 and WebAssembly 🕸](https://rustwasm.github.io/book/)

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