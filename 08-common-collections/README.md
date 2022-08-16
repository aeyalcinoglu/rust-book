Unlike the built-in array and tuple types, the data that the collections in the standard library point to is stored on the heap.

# Vectors
- Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. Vectors can only store values of the same type.
- Attempting to add an element to a vector while holding a reference to an item results in an error.

# Strings
- The method `to_string` is available on any type that implements the `Display` trait, as string literals do.
- Compiler can coerce the `&String` argument into a `&str`.
- A `String` is a wrapper over a `Vec<u8>`.
- An index into the string’s bytes will not always correlate to a valid Unicode scalar value. Rust doesn’t allow us to index into a `String` to get a character.
- One can iterate over strings via `char()` or `bytes()`.

# Hash Maps
- Like vectors, hash maps are homogeneous: all of the keys must have the same type, and all of the values must have the same type.
- For types that implement the `Copy` trait, like `i32`, the values are copied into the hash map. For owned values like `String`, the values will be moved and the hash map will be the owner of those values.
- By default, `HashMap` uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks involving hash tables.
- A *hasher* is a type that implements the `BuildHasher` trait.
