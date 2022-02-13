#![allow(unused)]

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s1 is moved and can no longer be used
    let s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // no ownership is taken
    let s: String = format!("{}-{}-{}", s1, s2, s3);

    // compiles but panics
    let hello: &str = "Здравствуйте";
    let s: &str = &hello[0..3];
    println!("{}", s);
}
