# OOP
- Objects came from [Simula](https://en.wikipedia.org/wiki/Simula) in the 1960s.
- [Bible of OOP, GoF](https://en.wikipedia.org/wiki/Design_Patterns)
- Rust doesn't have inheritance. However, default implementations of methods on traits can be used. For polymorphism, Rust has trait objects and generics.

# Trait objects
- A trait object points to both an instance of a type implementing our specified trait as well as a table used to look up trait methods on that type at runtime.
- A generic type parameter can only be substituted with one concrete type at a time, whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime.
- This concept — of being concerned only with the messages a value responds to rather than the value’s concrete type — is similar to the concept of *duck typing* in dynamically typed languages: if it walks like a duck and quacks like a duck, then it must be a duck!
- When we use trait objects, Rust must use *dynamic dispatch*, as opposed to using *static dispatch* when we use trait bounds on generics.
- At runtime, Rust uses the pointers inside the trait object to know which method to call. There is a runtime cost when this lookup happens that doesn’t occur with static dispatch. Also it prevents some compiler optimizations.

# State Pattern
- The *state pattern* is an object-oriented design pattern. It is a behavioral software design pattern that allows an object to alter its behavior when its internal state changes.
- See [object safety](https://doc.rust-lang.org/reference/items/traits.html#object-safety).
