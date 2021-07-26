mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Listing 7-11: Bringing a module into scope with `use`
use crate::front_of_house::hosting;

// Listing 7-17: Making a name available for any code to use from a new scope
//               with `pub use`
// pub use crate::front_of_house::hosting;

// Listing 7-12: Bringing a module into scope with `use` and a relative path
// use self::front_of_house::hosting;

// Listing 7-13: Bringing the `add_to_waitlist` function into scope with `use`,
//               which is unidiomatic
// use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// Listing 7-14: Bringing `HashMap` into scope in an idiomatic way
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}


// Listing 7-15: Bringing two types with the same name into the same scope
//               requires using their parent modules.
use std::fmt;
use std::io;

// Listing 7-18: Specifying a nested path to bring multiple items
//               with the same prefix into scope
// use std::{fmt, io};

fn function1_15() -> fmt::Result {
    // --snip--
    fmt::Result::Ok(())
}

fn function2_15() -> io::Result<()> {
    // --snip--
    io::Result::Ok(())
}

// Listing 7-16: Renaming a type when it’s brought into scope
//               with the `as` keyword
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1_16() -> Result {
    // --snip--
    Result::Ok(())
}

fn function4_16() -> IoResult<()> {
    // --snip--
    IoResult::Ok(())
}
