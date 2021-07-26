// -----------------------------------------------------------------------------
// 関数ポインタ
// Listing 19-27: Using the fn type to accept a function pointer as an argument
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn do_twice2<T>(f: T, arg: i32) -> i32
    where T: Fn(i32) -> i32
{
    f(arg) + f(arg)
}

/*
cargo test -- --nocapture 2>/dev/null
*/
#[test]
fn do_do_twice() {
    let answer1_1 = do_twice(add_one, 5);
    let answer1_2 = do_twice(|x| x + 1, 5);
    let answer2_1 = do_twice2(add_one, 5);
    let answer2_2 = do_twice2(|x| x + 1, 5);
    assert_eq!(answer1_1, answer1_2);
    assert_eq!(answer1_2, answer2_1);
    assert_eq!(answer2_1, answer2_2);
    println!("The answer is: {}", answer2_2);
}


fn use_closure() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();
}

// クロージャの代わりに関数を利用
fn use_function() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();;
}

// -----------------------------------------------------------------------------
// クロージャを返却する

/* ❌ コンパイル不可
fn returns_closure() -> Fn(i32) -> i32 {
    |x| x + 1
}*/

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
