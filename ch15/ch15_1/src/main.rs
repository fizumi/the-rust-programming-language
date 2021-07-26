mod list;

fn main() {
    listing_15_1();
}


fn listing_15_1() {
    let b = Box::new(5);
    println!("b = {}", b);
    println!("b = {}", *b);
}

