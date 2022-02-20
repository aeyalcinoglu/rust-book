// gives an error, even though the
// top-level Cargo.lock has rand info
// need to add it to local Cargo.toml
// use rand;
use add_one;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
}
