mod blog;
mod blog2;
mod average;
mod draw;

fn main() {
    state_pattern();
    another_pattern();
}

fn state_pattern() {
    let mut post = blog::Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

fn another_pattern() {
    let mut post = blog2::Post::new();

    post.add_text("I ate a salad for lunch today");
    // assert_eq!("", post.content());  // ⭕ コンパイルが通らない

    let post = post.request_review();
    // assert_eq!("", post.content()); // ⭕ コンパイルが通らない

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}