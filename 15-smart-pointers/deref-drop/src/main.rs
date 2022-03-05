use crate::List::{Cons, Nil};
use std::mem;
use std::ops::Deref;

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(
        -10000,
        Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Nil))))))),
    );
    // let list = Nil;
    println!(
        "list = {:?} has size of {} bytes.",
        list,
        mem::size_of_val(&list)
    );
    println!("List enum has size of {} bytes.", mem::size_of::<List>());

    let x = 5;
    // let y = &x; or Box::new(x) or
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    // need to call hello(&(*m)[..]) without coercions
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    // this is std::mem::drop
    // calling c.drop() would give the error
    // explicit destructor calls not allowed
    drop(c);
    println!("CustomSmartPointers created.");
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}
