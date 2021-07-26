// Defining Our Own Smart Pointer

// Listing 15-8: Defining a MyBox<T> type
struct MyBox<T>(T);


impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Listing 15-10: Implementing Deref on MyBox<T>
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
use super::*;
// Listing 15-9: Attempting to use MyBox<T> in the same way we used references and Box<T>
#[test]
fn listing_15_9() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5,*(y.deref()));
}
}
