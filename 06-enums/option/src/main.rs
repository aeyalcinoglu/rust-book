#![allow(unused_variables)]

fn main() {
    // Option<T> is already in scope
    // Also 'Option::' can be dropped
    let some_number = Some(5);
    println!("{:?}", some_number);
    // here typing the variable is necessary
    // since <T> cannot be inferred
    let absent_number: Option<i32> = None;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("The maximum is not configured");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
