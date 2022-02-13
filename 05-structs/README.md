# Structs
- A *struct*, or *structure*, is a custom data type that lets you name and package together multiple related values that make up a meaningful group.
- Rust doesn’t allow us to mark only certain fields as mutable.
- While using *struct update syntax*, the base struct must always be the last field.
- If all of the types of the fields of a struct implement the `Copy` trait, then the struct is stack-only data, and can be used after moving. This also applies to partial moving with struct update syntax.
- *Tuple structs* just have the types of the fields.
- In order to be able to store references in structs, one needs to specify *lifetimes*.
- A *methods* first parameter is always `self`, which represents the instance of the struct the method is being called on.
- The `&self` is actually short for `self: &Self`. Within an `impl` block, the type `Self` is an alias for the type that the `impl` block is for.
- Rust automatically adds in `&`, `&mut`, or `*` so `object` matches the signature of the method. The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.
- All functions defined within an `impl` block are called *associated functions* because they’re associated with the type named after the `impl`.
- The `::` syntax is used for both associated functions and namespaces created by modules.
- Each struct is allowed to have multiple `impl` blocks.