fn main() {
    let s1 = String::from("hello");
    // we need to pass the reference to make sure
    // that we can use s1 in the future
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hi");
    // change function will mutate the value it borrows
    change(&mut s2);

    // a reference's scope starts from where it is
    // introduced and continues through the last time
    // that reference is used
    let mut s = String::from("foo");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after
    // this point
    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
