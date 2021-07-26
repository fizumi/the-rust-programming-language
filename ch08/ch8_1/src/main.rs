fn main() {
    listing_8_1_to_8_5();
    listing_8_8();
    listing_8_9();
}

fn listing_8_1_to_8_5() {
    // Listing 8-1: Creating a new, empty vector to hold values of type `i32`
    let v: Vec<i32> = Vec::new();

    // Listing 8-2: Creating a new vector containing values
    let v = vec![1, 2, 3];

    // Listing 8-3: Using the `push` method to add values to a vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Listing 8-5: Using indexing syntax or the `get` method to access an item in a vector
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

#[cfg(test)]
mod listing_8_6 {
    #[test]
    #[should_panic(expected="index out of bounds:")]
    fn indexing_syntax() {
        // Listing 8-6: Attempting to access the element at index 100
        //              in a vector containing five elements
        let v = vec![1, 2, 3, 4, 5];

        let does_not_exist = &v[100];
    }
    #[test]
    fn get_method() {
        // Listing 8-6: Attempting to access the element at index 100
        //              in a vector containing five elements
        let v = vec![1, 2, 3, 4, 5];

        let does_not_exist = v.get(100);
        assert_eq!(does_not_exist, None);
    }
}


// Listing 8-7: Attempting to add an element to a vector
//              while holding a reference to an item
fn listing_8_7() {
    let mut v = vec![1, 2, 3, 4, 5];

//               ↓ immutable borrow occurs
    let first = &v[0];

//     ↓ mutable borrow occurs
    // v.push(6);

//                                       ↓ immutable borrow later used
    println!("The first element is: {}", first);
}

// Listing 8-8: Printing each element in a vector
//              by iterating over the elements using a for loop
fn listing_8_8() {

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}

// Listing 8-9: Iterating over mutable references to elements in a vector
fn listing_8_9() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
}

// Listing 8-10: Defining an enum to store values of different types in one vector
fn listing_8_10() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

// トレイトオブジェクト + ベクタ ⇒ Listing 17-9