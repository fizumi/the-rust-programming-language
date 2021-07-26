fn main() {
    // Listing 8-11: Creating a new, empty String
    let mut s = String::new();


    // Listing 8-12: Using the to_string method to create a String
    //               from a string literal
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();


    // Listing 8-13: Using the String::from function to create a String
    //               from a string literal
    let s = String::from("initial contents");
    // this code is equivalent to the code that uses `to_string`


    // Listing 8-15: Appending a string slice to a String using the `push_str` method
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("8-15: s is {}", s);


    // Listing 8-16: Using a string slice after appending its contents to a String
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("8-16: s2 is {}", s2);


    // Listing 8-17: Adding one character to a String value using push
    let mut s = String::from("lo");
    s.push('l');
    println!("8-17: s is {}", s);


    // Listing 8-18: Using the + operator to combine two String values into a new String value
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here
    println!("8-18: s3 is {}", s3);
    // s1 can no longer be used
    // println!("8-18: s1 is {}", s1);

    // No List:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("`+`operator: s, s2, s3: {}, {}, {}", s, s2, s3);
    // println!("s1: {}", s1); // s1 can no longer be used

    // `format!` is much easier to read and doesn’t take ownership
    let s1 = String::from("tic");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("`format!`: s, s1, s2, s3: {}, {}, {}, {}", s, s1, s2, s3);

    methods_for_iterating_over_strings();
}

#[cfg(test)]
mod slicing_strings {
    #[test]
    fn valid_index() {
        let hello = "Здравствуйте";
        let s = &hello[0..4];
        assert_eq!(s, "Зд");
    }
    #[test]
    #[should_panic(expected="not a char boundary")]
    fn invalid_index() {
        let hello = "Здравствуйте";
        let s = &hello[0..1];
    }
}

fn methods_for_iterating_over_strings() {
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}