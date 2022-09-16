# Unsafe Rust
- Unsafe Rust exists because, by nature, static analysis is conservative.
- Another reason Rust has an unsafe alter ego is that the underlying computer hardware is inherently unsafe.
- There are 5 *unsafe superpowers*:
	- Dereference a raw pointer
	- Call an unsafe function or method
	- Access or modify a mutable static variable
	- Implement an unsafe trait
	- Access fields of `union`s
- To interact with code written in another language, Rust has a keyword, `extern`, that facilitates the creation and use of a *Foreign Function Interface (FFI)*.
- Difference between constants and immutable static variables is that values in a static variable have a fixed address in memory.
- Accessing and modifying mutable static variables is *unsafe*.
- See [the reference on union](https://doc.rust-lang.org/stable/reference/items/unions.html).
- Using `unsafe` to take one of the five actions (superpowers) just discussed isn’t wrong or even frowned upon, but it is trickier.

# Advanced Traits
- *Associated types* connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures.
- With associated types, one doesn't need to annotate types because one can’t implement a trait on a type multiple times.
- Use `<PlaceholderType=ConcreteType>` syntax to specify a default concrete type for the generic type.
- Rust doesn't always know which type you mean unless you use *fully qualified syntax*. In general it is defined as:
```rust
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```
- The trait your trait definition is relying on is called a *supertrait* of the trait.

# Advanced Types
- Rust provides the ability to declare a *type alias* to give an existing type another name. For this we use the `type` keyword.
- Rust has a special type named `!` that’s known in type theory lingo as the *empty type* because it has no values. In Rust it is called the *never type* because it stands in the place of the return type when a function will never return. Functions that return never are called *diverging functions*.

# Advanced Functions and Closures
- Functions coerce to the type `fn` (with a lowercase f), not to be confused with the `Fn` closure trait. The `fn` type is called a *function pointer*.
- Closures are represented by traits, which means one can’t return closures directly.

# Macros
- The term *macro* refers to a family of features in Rust: *declarative* macros with `macro_rules!` and three kinds of *procedural* macros:
	- Custom `#[derive]` macros that specify code added with the derive attribute used on structs and enums
	- Attribute-like macros that define custom attributes usable on any item
	- Function-like macros that look like function calls but operate on the tokens specified as their argument
- Fundamentally, macros are a way of writing code that writes other code, which is known as *metaprogramming*.
- A function signature must declare the number and type of parameters the function has. Macros, on the other hand, can take a variable number of parameters.
- `derive` only works for structs and enums. 
