// Listing 10-6: A Point<T> struct that holds x and y values of type T
struct Point<T> {
    x: T,
    y: T,
}

// Listing 10-8: A Point<T, U> generic over two types so that
//               x and y can be values of different types
fn listing_10_6() {
    // let integer_and_float = Point { x: 5, y: 4.0 }; // ❌
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Listing 10-9: Implementing a method named x on the Point<T> struct
//               that will return a reference to the x field of type T
fn listing_10_9() {
    let p = Point { x: 5, y: 10 };

    let x = p.x();
    println!("p.x = {}", x);
}

// Listing 10-10: An impl block that only applies to a struct with a particular
//                concrete type for the generic type parameter T
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn listing_10_10() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };

    // let x = both_integer.distance_from_origin(); // ❌ can't use
    let x = both_float.distance_from_origin();
}