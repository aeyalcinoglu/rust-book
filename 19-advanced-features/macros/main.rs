use hello_macro::HelloMacro;

struct Pancekes;

fn main() {
    let a_vec: Vec<u32> = hello_macro::simple_vec![1, 2, 3];
    println!("{:?}", a_vec);
}
