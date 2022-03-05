# Smart Pointers
- A *pointer* is a general concept for a variable that contains an address in memory.
- *Smart pointers* are data structures that not only act like a pointer but also have additional metadata and capabilities.

# Box<T>
- Boxes allow you to store data on the heap rather than the stack.
- A *cons list* is a data structure from Lisp.
- A pointer is always `usize` (8 bytes on a 64bit system), where the size of an `i32` is 4 bytes.

# Deref Trait
- Implementing the `Deref` trait allows you to customize the behavior of the *dereference operator*, `*`.
- The `Deref` trait, provided by the standard library, requires one to implement one method named `deref` that borrows `self` and returns a reference to the inner data.
- *Deref coercion* is a convenience that Rust performs on arguments to functions and methods. Rust does deref coercion when it finds types and trait implementations in three cases:
	- From `&T` to `&U` when `T: Deref<Target=U>`
	- From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
	- From `&mut T` to `&U` when `T: Deref<Target=U>`
  
For context: The first case states that if you have a `&T`, and `T` implements `Deref` to some type `U`, you can get a `&U` transparently.

# Drop Trait
- `Drop` trait lets you customize what happens when a value is about to go out of scope.
- The `Drop` trait requires you to implement one method named `drop` that takes a mutable reference to `self`.
- Variables are dropped in the reverse order of their creation.
- Calling `Drop` trait's `drop` method manually gives a compiler error like `explicit destructor calls not allowed`. One should use `std::mem::drop`.

# Rc\<T\> - The Reference Counted Smart Pointer
- To enable multiple ownership, Rust has a type called `Rc<T>`, which is an abbreviation for *reference counting*.
- `Rc<T>` is only for use in single-threaded scenarios.
- The implementation of `Rc::clone` doesn't make a deep copy of all the data.

# RefCell\<T\>
- *Interior mutability* is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data.
- Unlike `Rc<T>`, the `RefCell<T>` type represents single ownership over the data it holds.
- With references and `Box<T>`, the borrowing rules' invariants are enforced at compile time. With `RefCell<T>`, these invariants are enforced at *runtime*.
- Also is only for use in single-threaded scenarios. 
