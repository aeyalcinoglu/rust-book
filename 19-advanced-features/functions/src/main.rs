#![allow(dead_code)]

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// not using Box results in the error
// "doesn't have a size known at compile-time"
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!(
        "List of strings {:?}\nList of statuses {:?}",
        list_of_strings, list_of_statuses
    );
}
