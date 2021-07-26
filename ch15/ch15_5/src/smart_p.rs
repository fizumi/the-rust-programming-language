
fn error() {
    let s1 = String::from("hello");
    let s2 = String::from(", world!");

    let s = s1 + &s2;

    // println!("{}", s1); // ❌
}

// https://doc.rust-jp.rs/book-ja/ch15-05-interior-mutability.html
#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    #[test]
    fn use_rc() {
        let s1 = Rc::new(RefCell::new(String::from("hello")));
        let s2 = String::from(", world!");

        use std::ops::Add;
        // println!("{:?}", *s1.borrow_mut().add(&s2)); // ❌ doesn't have a size known at compile-time
    }
}