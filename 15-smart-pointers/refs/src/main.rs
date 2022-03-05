use std::mem;

#[derive(Debug)]
enum List {
    // or just i32 if you don't need to
    // change any of the values in the list
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    // trying to handle this with Box
    // would give ownership error
    {
        let _c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    let rc_ref_nil = Rc::new(RefCell::new(Nil));
    let nil = Nil;
    println!(
        "Size of a, b, rc_ref_nil, nil are {}, {}, {}, {} bytes.",
        mem::size_of_val(&a),
        mem::size_of_val(&b),
        mem::size_of_val(&rc_ref_nil),
        mem::size_of_val(&nil)
    );
    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
}
