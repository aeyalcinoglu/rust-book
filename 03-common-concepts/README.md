# Rust is an expression-oriented language
A block has the form

```
{
    stmt;
    stmt;
    ...
    stmt;
    expr
}
```

The statements are (basically) expressions or `let` bindings, and the trailing expression is implicitly `()` if not specified. The value of the whole block is the value of this last expression.

# Variables
- By default variables are immutable.
- Trying to change the type of a mutable variable, without shadowing (without using the keyword `let`), would give a compliation error, e.g. `` expected `&str`, found `usize` ``.

# Data Types
- A *scalar* type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
- [The book is slightly imprecise about integer overflows](https://stackoverflow.com/a/70776258).
- Rust’s floating-point types are `f32` and `f64`.
- *Compound types* can group multiple values into one type. Rust has two primitive compound types: tuples and arrays. They both must have a fixed length, elements of tuples can have different types.

# Functions
- In function signatures, you must declare the type of each parameter.
- *Statements* are instructions that perform some action and do not return a value. *Expressions* evaluate to a resulting value.

# Control Flow
- `if` is an expression. The values that have the potential to be results from each arm of the `if` must be the same type.
- Rust has three kinds of loops: `loop`, `while`, and `for`.
