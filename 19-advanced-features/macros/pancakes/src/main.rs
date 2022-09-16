use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    let a_vec: Vec<u32> = pancakes::simple_vec![1, 2, 3];
    println!("{:?}", a_vec);

    Pancakes::hello_macro();
}
