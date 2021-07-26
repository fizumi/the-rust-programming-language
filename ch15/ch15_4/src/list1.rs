// Listing 15-17: Demonstrating we’re not allowed to have two lists using Box<T> that try to share ownership of a third list
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use self::List::{Cons, Nil};

fn listing_15_17() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a)); // ❌ use of moved value: `a`
}

