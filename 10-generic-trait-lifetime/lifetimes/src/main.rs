#![allow(unused)]

use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let string1 = String::from("abcd");
    // even if we declare result in outer scope
    // and string1 is longer, result still has the
    // smaller lifetime
    {
        let mut string2 = "xyz";
        string2 = first(string2, "foo");
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
        println!(
            "{}",
            longest_with_an_announcement(
                string1.as_str(),
                string2,
                String::from("Longest string is...")
            )
        );
    }

    // moby dick
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence: &str = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// gives "missing lifetime specifier"
// error without the lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// both lifetime annotations are necessary, no elision
fn first<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcment! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
