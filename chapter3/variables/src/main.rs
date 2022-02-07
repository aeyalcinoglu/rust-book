// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5; // remove mut to see the error message
    println!("The value of x is: {}", x);
    x = 6; // or have let here for shadowing
    println!("The value of x is: {}", x);

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner space is: {}", y);
    }

    println!("The value of y is: {}", y);

    // with shadowing, one can change the type of the value
    let spaces = "   "; // add mut to see the warning message
    let spaces = spaces.len(); // additionally, remove let to see the error message

    println!("Number of spaces: {}", spaces);
}
