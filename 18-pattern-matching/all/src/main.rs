#![allow(unused)]

struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Bet {
    Guess { id: i32 },
}

fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // Some(n) if n == y would make us able
        // to use outer y rather than introducing
        // a shadowed variable and skip this arm
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 7;

    match x {
        1 | 2 => println!("one or two"),
        // can also use char values, e.g. 'a'..='j'
        // but can't use in this match since
        // type should be consistent
        3..=7 => println!("three through seven"),
        // would get - patterns `i32::MIN..=0_i32`
        // and `8_i32..=i32::MAX` not covered error
        _ => println!("anything"),
    }

    let p = Point { x: 1, y: 7 };
    // this part is same in JS
    let Point { x: _a, y: b } = p;
    let Point { x, y } = p;
    assert_eq!(7, b);
    assert_eq!(8, x + y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // match guards
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let bet = Bet::Guess { id: 5 };
    match bet {
        Bet::Guess {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Bet::Guess { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Bet::Guess { id } => println!("Found some other id: {}", id),
    }
}
