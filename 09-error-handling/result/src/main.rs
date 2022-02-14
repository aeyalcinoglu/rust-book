#![allow(unused)]

use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    let filename = String::from("hello.txt");
    let username = String::from("foo");
    take_or_create_file(&filename);
    // write_somehow(filename, username)
    let read_username = read_username_from_file(&filename);
    match read_username {
        Ok(name) => println!("{}", name),
        Err(error) => panic!("{}", error),
    };
}

fn take_or_create_file(filename: &String) -> File {
    // or .expect()
    // let f1 = File::open("foo.txt").unwrap();

    let f = File::open(filename);
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", error)
            }
        },
    };
    return f;
}

// propagating errors with the ? operator
// can replace the body with
// fs::read_to_string(filename)
fn read_username_from_file(filename: &String) -> Result<String, io::Error> {
    let mut s = String::new();

    File::open(filename)?.read_to_string(&mut s)?;

    Ok(s)
}
