fn main() {
    listing_8_20();
    listing_8_21();
    listing_8_22();
    listing_8_23();
    listing_8_24();
    listing_8_25();
    listing_8_26();
}

// Listing 8-20: Creating a new hash map and inserting some keys and values
fn listing_8_20() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("8-20: {:?}", scores)
}

// Listing 8-21: Creating a hash map from a list of teams and a list of scores
fn listing_8_21() {
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // The type annotation HashMap<_, _> is needed here
    // because it’s possible to collect into many different data structures
    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("8-21: {:?}", scores)
}

// Listing 8-22: Showing that keys and values are owned
//               by the hash map once they’re inserted
fn listing_8_22() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("8-22: {:?}", map)
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // println!("8-22: {}, {}", field_name, field_value)
}

// Listing 8-23: Accessing the score for the Blue team stored in the hash map
fn listing_8_23() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("8-23: {:?}", score);

    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }
}

// Listing 8-24: Replacing a value stored with a particular key
fn listing_8_24() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("8-23: {:?}", scores);
}

// Listing 8-25: Using the entry method to only insert
//               if the key does not already have a value
fn listing_8_25() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("8-23: {:?}", scores);
}

// Listing 8-26: Counting occurrences of words
//               using a hash map that stores words and counts
fn listing_8_26() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
