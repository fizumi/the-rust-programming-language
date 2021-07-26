fn main() {
    let mut s = String::from("hello world");

    let word_len = first_word_size(&s);
    let word = first_word(&s);
                       // ↑ immutable borrow

//  ↓ mutable borrow
    // s.clear(); // error!

    println!("len of '{}' is: {}", word, word_len);
}

fn first_word_size(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


// fn first_word(s: &String) -> &str { // ❌文字列スライスを受け取ることが出来ない
fn first_word(s: &str) -> &str {       // ⭕文字列スライスを受け取ることが出来る
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn string_slices_as_parameters() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

fn other_slices() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3]; // slice: &[i32]

    assert_eq!(slice, &[2, 3]);
}