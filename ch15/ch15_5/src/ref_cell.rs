//------------------------------------------------------------------------------
// Mutable References
//------------------------------------------------------------------------------
fn multiple_immutable_references() {
    let mut s = String::from("hello");

    let r1 = &mut s;    // r1 reference’s scope start
//     let r2 = &mut s; // ❌ 借用規則違反

    println!("{}", r1); // r1 reference’s scope end
}

use std::cell::RefCell;
fn multiple_immutable_references_with_refcell() {
    let mut s = RefCell::new(String::from("hello"));

    let r1 = s.borrow_mut();
    let r2 = s.borrow_mut(); // ⭕ // コンパイルは通る。しかし、実行時エラーとなる
}

//------------------------------------------------------------------------------
// Combine References
//------------------------------------------------------------------------------
fn combine_references() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
//     let r2 = &mut s; // ❌ 借用規則違反

//     println!("{} and {}", r1, r2);
}

fn combine_references_with_refcell() {
    let mut s = RefCell::new(String::from("hello"));

    let r1 = s.borrow();
    let r2 = s.borrow_mut();
}


#[cfg(test)]
mod tests {
#[test]
#[should_panic(expected="already borrowed: BorrowMutError")] // ❗
fn multiple_immutable_references() {
    super::multiple_immutable_references_with_refcell();
}
#[test]
#[should_panic(expected="already borrowed: BorrowMutError")] // ❗
fn combine_references() {
    super::combine_references_with_refcell();
}
}