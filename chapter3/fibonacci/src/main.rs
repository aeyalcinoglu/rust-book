use std::io;

fn main() {
    let mut n = String::new(); // must use mut, or error: cannot borrow as mutable

    println!("Enter a non-negative integer n:");

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read the input.");

    let n: usize = n.trim().parse().expect("Failed to parse input.");

    println!("{}. Fibonacci number is {}", n, fibonacci(n));
}

// warning: exponential run time
fn fibonacci(n: usize) -> usize {
    if n == 0 || n == 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
