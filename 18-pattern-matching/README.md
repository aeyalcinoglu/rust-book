# General
- Patterns are a special syntax in Rust for matching against the structure of types.
- A pattern consists of some combination of the following:
	- Literals
	- Destructured arrays, enums, structs, or tuples
	- Variables
	- Wildcards
	- Placeholders
- Formally, a `let` statement looks like this:
  
```
let PATTERN = EXPRESSION;
```

# Refutability
- Patterns come in two forms: refutable and irrefutable.
- Patterns that will match for any possible value passed are irrefutable. For example `let x = 5;`. Function parameters, `let` statements, and `for` loops can only accept irrefutable patterns.
- Patterns that can fail to match for some possible value are refutable. For example `if let Some(x) = a_value`. The `if let` and `while let` expressions accept refutable and irrefutable patterns.

# More
- In `match` expressions, you can match multiple patterns using the `|` syntax, which means *or*.
- The syntax `..` will expand to as many values as it needs to be.
- A *match guard* is an additional `if` condition specified after the pattern in a `match` arm that must also match, along with the pattern matching, for that arm to be chosen.
- The *at* operator (`@`) lets us create a variable that holds a value at the same time we're testing that value to see whether it matches a pattern. 
