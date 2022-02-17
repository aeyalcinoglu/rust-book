#![allow(unused)]

struct Sentence {
    _back: String,
    front: String,
}

struct SentenceUp<'a> {
    _back: String,
    front: &'a String,
}

impl<'a> SentenceUp<'a> {
    fn get_front_up(&'a self) -> &'a String {
        self.front
    }
}

fn main() {
    let sentence = Sentence {
        _back: String::from("foo"),
        front: String::from("bar"),
    };
    let sentence_up = SentenceUp {
        _back: String::from("foo_up"),
        front: &String::from("bar_up"),
    };

    let front_of_sentence = get_front4(&sentence);
    println!("{}", front_of_sentence);

    // check ownership
    println!("{}", sentence.front);

    let front_of_sentence_up = sentence_up.get_front_up();
    println!("{}", front_of_sentence_up);
    println!("{}", sentence_up.front);
}

// all four working functions result in the same
// output, but, first one eats the ownership
fn get_front1(sentence: Sentence) -> String {
    sentence.front
}

fn get_front2(sentence: &Sentence) -> &String {
    &sentence.front
}

fn get_front3(sentence: &Sentence) -> String {
    format!("{}", sentence.front)
}

fn get_front4(sentence: &Sentence) -> String {
    let sentence_front = ["", &sentence.front].join("");
    sentence_front
}

// fn wrong_get_front(sentence: &Sentence) -> String {
//     sentence.front
// }
