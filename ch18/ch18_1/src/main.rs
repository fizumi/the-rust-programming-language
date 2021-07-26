fn main() {

    // -------------------------------------------------------------------------
    // Listing 18-1: Mixing if let, else if, else if let, and else

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }


    // -------------------------------------------------------------------------
    // Listing 18-2: Using a while let loop to print values for as long as stack.pop() returns Some

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }


    // -------------------------------------------------------------------------
    // Listing 18-3: Using a pattern in a for loop to destructure a tuple

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }


    // -------------------------------------------------------------------------
    // Listing 18-6: A function signature uses patterns in the parameters
    // fn foo(x: i32) { --snip-- }

    // -------------------------------------------------------------------------
    // Listing 18-7: A function with parameters that destructure a tuple
    let point = (3, 5);
    print_coordinates(&point);
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

}

