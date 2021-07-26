fn main() {
    listing_10_20();
}

// Listing 10-20: A main function that calls the longest function
//                to find the longer of two string slices
fn listing_10_20() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// Listing 10-22: The longest function definition specifying that all the
//                references in the signature must have the same lifetime 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Listing 10-23: Using the longest function with references to String values
//                that have different concrete lifetimes
fn listing_10_23() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

// Listing 10-24: Attempting to use result after string2 has gone out of scope
fn listing_10_24() {
    let string1 = String::from("long string is long");
    // let result;
    {
        let string2 = String::from("xyz");
        // result = longest(string1.as_str(), string2.as_str());
    }
    // println!("The longest string is {}", result);
}


// Generic Type Parameters, Trait Bounds, and Lifetimes Together
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}