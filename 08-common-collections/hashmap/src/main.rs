#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut collected_scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // doesn't panic
    assert_eq!(scores, collected_scores);

    // does nothing if the key exists
    // for force update, just use insert
    // also, the return type is &mut V
    scores.entry(String::from("Yellow")).or_insert(40);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
