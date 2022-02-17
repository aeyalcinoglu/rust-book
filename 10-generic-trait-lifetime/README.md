# Generics
- *Generics* are abstract stand-ins for concrete types or other properties.
- Rust implements generics in such a way that your code doesn’t run any slower using generic types than it would with concrete types. Rust accomplishes this by performing monomorphization of the code that is using generics at compile time. *Monomorphization* is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

# Traits
- A *trait* tells the Rust compiler about functionality a particular type has and can share with other types.
- The trait has to be brought into scope as well as the types to get the additional trait methods.
- We can’t implement external traits on external types.
- The visibility of a trait implementation is the intersection of the visibility of the trait and the visibility of the type it’s implemented on.
- It isn’t possible to call the default implementation from an overriding implementation of that same method.
- One can only use `impl Trait` if you’re returning a single type.
- By using a trait bound with an `impl` block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.
- We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called *blanket implementations* and are extensively used in the Rust standard library.

# Lifetimes
- Every reference in Rust has a *lifetime*, which is the scope for which that reference is valid. Most of the time, lifetimes are implicit and inferred.
- The Rust compiler has a *borrow checker* that compares scopes to determine whether all borrows are valid.
- Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
```rust
&'a mut i32 // a mutable reference with an explicit lifetime
```
- Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.
- The patterns programmed into Rust’s analysis of references are called the *lifetime elision rules*.
- The compiler uses three rules to figure out what lifetimes references have when there aren’t explicit annotations. If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error.
	- Each parameter that is a reference gets its own lifetime parameter.
	- If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
	- If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all output lifetime parameters.
- One special lifetime is `'static'`, which means that this reference *can* live for the entire duration of the program. All string literals have the `'static'` lifetime.
