fn main() {
    let s = String::from("hello world");
    // omitting & would give the error
    // the size for values of type `str`
    // cannot be known at compilation time
    let hello = &s[0..5];
    println!("{}", hello);

    let sentence: String = String::from("How are you?");
    let word: &str = first_word(&sentence);
    // making sentence mutable and running the next line
    // of code produces error, since word is an immutable
    // reference to the underlying data and
    // adding mut to everywhere gives second mutable
    // borrow error
    // sentence.clear();
    println!("{}", word);

    let a = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

// here we are doing deref coercion by taking &str
// as the type of the parameter s, instead of &String
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
