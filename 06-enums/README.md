# Enums
*enumerations*, also referred to as *enums*, allows one to define a type by enumerating its possible *variants*.

## Option
- Rust doesn't have the null feature. See [Null References: The Billion Dollar Mistake](https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare/)
- Rust has an enum that can encode the concept of a value being present or absent. This enum is `Option<T>`, and it is defined by the standard library as follows:
```rust
enum Option<T> {
    None,
    Some(T),
}
```
- You have to convert an `Option<T>` to a `T` before you can perform `T` operations with it. Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.

## Match
- Matches in Rust are *exhaustive*: we must exhaust every last possibility in order for the code to be valid.
- The code associated with each arm is an expression.
- Rust also has a pattern we can use when we don’t want to use the value in the catch-all pattern: `_`, which is a special pattern that matches any value and does not bind to that value.
- `_ => ()` says "catch-all and do nothing".
- One can think of `if let` as syntax sugar for a `match` that runs code when the value matches one pattern and then ignores all other values. 
