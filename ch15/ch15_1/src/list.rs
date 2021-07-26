// Listing 15-2: The first attempt at defining an enum to represent a cons list data structure of i32 values
// enum List {  // ❌ recursive type `list::List` has infinite size
//     Cons(i32, List),
//     Nil,
// }
// Listing 15-3: Using the List enum to store the list 1, 2, 3
// fn listing_15_3() {
//     let list = Cons(1, Cons(2, Cons(3, Nil)));
// }

// Listing 15-5: Definition of List that uses Box<T> in order to have a known size
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn listing_15_5() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}