// these are crate-level allow attributes
// to surprass warnings, notice '!'
#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // tuple struct
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    // unit-like struct
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}

// use #[derive(Debug)] to be able to be
// able to print instances with {:?}
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// here we are using the field init shorthand syntax
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
