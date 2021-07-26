// Listing 10-16: Conditionally implement methods on a generic type
//                depending on trait bounds
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

pub fn listing_10_16() {
    let p1 = Pair::new(1, 2);
    let p2 = Pair::new(Pair::new(1, 2), Pair::new(3, 4));

    p1.cmp_display();
    // p2.cmp_display(); // ❌ // Display + PartialOrd を満たさない
}
