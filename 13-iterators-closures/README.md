- Closures and iterators are Rust features inspired by functional programming language ideas. They contribute to Rust’s capability to clearly express high-level ideas at low-level performance.
- *Closures*, a function-like construct you can store in a variable.
- *Iterators*, a way of processing a series of elements.

# Closures
- Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.
- Closures don’t require you to annotate the types of the parameters or the return value like `fn` functions do.
- If there is no type annotation, then the closure must be used, and types will be locked after the first usage.
- One can create a struct that will hold the closure and the resulting value of calling the closure. The struct will execute the closure only if we need the resulting value, and it will cache the resulting value so the rest of the code doesn't have to be responsible for saving and resulting the result. This pattern is called *memoization* or *lazy evaluation*.
- Each closure has its own unique anonymous type.
- All closures implement at least one of the traits: `Fn`, `FnMut`, or `FnOnce`.
- Closures can capture values from their environment in three ways. These are encoded in the three `Fn` traits as follows:
	- `FnOnce`: Consumes the variables it captures from its enclosing scope. All closures implement `FnOnce`.
	- `FnMut`: Can change the environment because it mutably borrows values. Closures that don't move the captured variables implement `FnMut`.
	- `Fn`: Borrows values from the environment immutably.
- [How can a closure using the `move` keyword create a `FnMut` closure?](https://stackoverflow.com/questions/50135871/how-can-a-closure-using-the-move-keyword-create-a-fnmut-closure)

# Iterators
- In Rust, iterators are *lazy*, meaning they have no effect until you call methods that consume the iterator to use it up.
- All iterators implement a trait named `Iterator` that is defined in the standard library. It uses *associated types*, but only requires implementation of the `next` method.
- Methods on the `Iterator` trait with default implementations provided by the standard library which call `next` are called *consuming adaptors*. Other methods are known as *iterator adaptors*.
- Iterators, although a high-level abstraction, get compiled down to roughly the same code as if you’d written the lower-level code yourself. Iterators are one of Rust’s *zero-cost abstractions*.
- Rust knows how many iterations are there and "unrolls" the loop.
