use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    let v: Vec<i32> = vec![1, 2, 3];

    let handle: JoinHandle<()> = thread::spawn(move || {
        // need move for v
        for i in 1..10 {
            println!("hi number {} from the handle thread!", i);
        }
        println!("Here's a vector: {:?}", v);
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // drop(v) now creates the usual borrow error
    handle.join().unwrap();
}
