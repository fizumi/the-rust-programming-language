mod my_box;
// Listing 15-6: Using the dereference operator to follow a reference to an i32 value
fn main() {

}


#[cfg(test)]
mod tests {
#[test]
fn listing_15_6() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
}
