use std::fmt;
use std::ops::Add;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

trait FakeGeneric<T> {
    fn fake_method(&mut self) -> Option<T>;
}

impl FakeGeneric<String> for Point {
    fn fake_method(&mut self) -> Option<String> {
        Some(String::from("What is the point"))
    }
}

impl FakeGeneric<i32> for Point {
    fn fake_method(&mut self) -> Option<i32> {
        Some(self.x)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let mut point: Point = Point { x: 42, y: 1 };
    // change i32 to String to get a different implementation
    println!("{}", FakeGeneric::<i32>::fake_method(&mut point).unwrap());

    let person = Human;
    Wizard::fly(&person);
    person.fly();

    println!(
        "A dog is called {} or {}.",
        Dog::baby_name(),
        <Dog as Animal>::baby_name()
    );

    point.outline_print();
}
