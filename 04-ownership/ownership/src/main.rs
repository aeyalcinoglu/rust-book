fn main() {
    // string literals are immutable.
    // let mut t = "hello";
    // here we just change what's being referred to,
    // not the string literal itself
    // t = "world";

    // a String holding the value "hello" *bound* to s
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // this creates a copy of the value
    let x = 5;
    let y = x;
    println!("{}", y);

    // this only copies the String data on the stack
    let s1 = String::from("foo");
    // use s1.clone() to escape from invalidation, but this
    // duplicates the heap data
    let mut s2 = s1;
    // s1 is invalidated
    println!("{}", s2);

    s2 = takes_ownership(s2);
    // s2 is moved into the function and is no longer valid
    // so the following line of code would give an error
    // if we were not returning the ownership back
    println!("{}", s2);
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}
