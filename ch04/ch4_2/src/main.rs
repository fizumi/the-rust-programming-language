fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

//                     ↓ borrow
fn calculate_length(s: &String) -> usize {
    s.len()
}

//------------------------------------------------------------------------------
// Mutable References
//------------------------------------------------------------------------------
fn mutable_references() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("s is {}.", s);
}

//                      ↓ mutable reference
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn cannot_borrow_s_as_mutable_more_than_once_at_a_time() {
    let mut s = String::from("hello");

    let r1 = &mut s;    // r1 reference’s scope start
    // let r2 = &mut s; // ❌

    println!("{}", r1); // r1 reference’s scope end
}

fn can_have_multiple_mutable_references_because_of_reference_scope() {
    let mut s = String::from("hello");

    let r1 = &mut s;    // r1 reference’s scope start
    println!("{}", r1); // r1 reference’s scope end

    let r2 = &mut s;
    println!("{}", r2);
    // println!("{}", r1); // ← これを追加すると r1 の参照スコープが伸び、
                           //   r2 の作成はエラーとなる
}

fn make_a_new_mutable_reference_not_simultaneously() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s; // ⭕
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

//------------------------------------------------------------------------------
// Immutable References
//------------------------------------------------------------------------------
fn can_have_multiple_immutable_references_simultaneously() {
    let s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &s; // no problem

    println!("{}, {}, {}", r1, r2, r3);
}

//------------------------------------------------------------------------------
// Combine References
//------------------------------------------------------------------------------
fn cannot_combine_mutable_and_immutable_references() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    // let r2 = &mut s; // ❌ BIG PROBLEM

    // println!("{} and {}", r1, r2);
}

fn combinng_mutable_and_immutable_references() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem       // r1 reference’s scope start
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);   // r1 reference’s scope end
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}


//------------------------------------------------------------------------------
// Dangling References
//------------------------------------------------------------------------------
/*
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
*/
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}