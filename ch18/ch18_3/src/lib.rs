/*
cargo test matching_literals -- --nocapture 2>/dev/null
*/
#[test]
fn matching_literals() {
    // let x = 1;
    let x = 4;
    // let x = '4';

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

/*
cargo test matching_named_variables -- --nocapture 2>/dev/null
*/
#[test]
fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

#[test]
fn multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

#[test]
fn matching_ranges_numeric_values() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"), // 1 | 2 | 3 | 4 | 5
        _ => println!("something else"),
    }
}


#[test]
fn matching_ranges_char_values() {
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

/*
cargo test destructuring_structs__ -- --nocapture 2>/dev/null
*/
#[test]
fn destructuring_structs__() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p; // shorthand
    assert_eq!(0, x);
    assert_eq!(7, y);

    // Destructuring and matching literal values in one pattern
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

/*
cargo test destructuring_enum_variants -- --nocapture 2>/dev/null
*/
#[test]
fn destructuring_enum_variants_that_hold_different_kinds_of_values() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }
}

/*
cargo test structs_and_enums -- --nocapture 2>/dev/null
*/
#[test]
fn destructuring_nested_structs_and_enums() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}


#[test]
fn destructuring_structs_and_tuples() {
    struct Point {
        x: i32,
        y: i32,
    }
    // 変数名をアンダースコアで始めることで、 コンパイラに未使用変数について警告しないよう指示することができる
    let ((_feet, _inches), Point { x: _, y: _ }) = ((3, 10), Point { x: 3, y: -10 });
}

#[test]
fn ignoring_an_entire_value() {
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }
    foo(3, 4);
}

/*
cargo test ignoring_parts_of_a_value -- --nocapture 2>/dev/null
*/
#[test]
fn ignoring_parts_of_a_value() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

/*
cargo test ignoring_multiple_parts_of_a_tuple -- --nocapture 2>/dev/null
*/
#[test]
fn ignoring_multiple_parts_of_a_tuple() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
}

#[test]
fn an_unused_variable_starting_with_an_underscore() {
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    // ⚠ 「_」だけの場合、束縛しない ⇒ `_s` が _ なら以下はエラーにならない
    // println!("{:?}", s); // (E0382)
}

#[test]
fn ignoring_all_fields_of_a_point_except_for_x() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}

#[test]
fn matching_only_the_first_and_last_values() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}

/*
cargo test adding_a_match_guard_to_a_pattern -- --nocapture 2>/dev/null
*/
#[test]
fn adding_a_match_guard_to_a_pattern() {
    let num = Some(4);
    let num = Some(6);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        Some(x) if x > 5 => println!("greater than five: {}", x), // ❌ unreachable pattern
        None => (),
    }
}

#[test]
fn using_a_match_guard_to_test_for_equality_with_an_outer_variable() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
}

/*
cargo test combining_multiple_patterns_with_a_match_guard -- --nocapture 2>/dev/null
*/
#[test]
fn combining_multiple_patterns_with_a_match_guard() {
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

/*
cargo test at_mark -- --nocapture 2>/dev/null
*/
#[test]
fn using_at_mark_to_bind_to_a_value_in_a_pattern_while_also_testing_it() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };
    // let msg = Message::Hello { id: 12 };
    // let msg = Message::Hello { id: 13 };

    match msg {
        // Message::Hello { // ❌ 文法エラー
        //     id @ 1,
        // } => println!("Found an id in range: {}", id),
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}



