fn main() {
    let x = 5;
    another_function(x);

    let y = plus_one(x);
    println!("The value of y is: {}", y);

    let z = empty_function();
    println!(
        "Function without return type returns: {:?} - the unit type",
        z
    );
}

fn another_function(x: i32) {
    let y = {
        let z = 3;
        x + z
    };

    println!("The value of x is: {}", y - 3);
}

fn plus_one(mut x: i32) -> i32 {
    x = x + 1;
    x
}

fn empty_function() {
    // can include statements, like another_function
}
