use blog::reblog::Post as RePost;
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    let mut bad_post = Post::new();

    bad_post.add_text("Or did I?");
    bad_post.request_review();
    bad_post.reject();
    bad_post.approve();
    assert_ne!("Or did I?", bad_post.content());

    let mut re_post = RePost::new();
    re_post.add_text("I ate a salad for lunch today");
    let re_post = re_post.request_review();
    // would create a compile-time error
    // let re_post = re_post.reject();
    let re_post = re_post.approve();
    assert_eq!("I ate a salad for lunch today", re_post.content());

    println!("Didn't panic!");
}
