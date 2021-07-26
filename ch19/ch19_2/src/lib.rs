//------------------------------------------------------------------------------
// Addトレイトを実装してPointインスタンス用に+演算子をオーバーロードする

// Listing 19-14: Implementing the Add trait to overload the + operator for Point instances
use std::ops::Add; /* https://doc.rust-lang.org/std/ops/trait.Add.html
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}*/

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Addトレイトにはデフォルト型引数（trait Add<RHS=Self> {--snip--}）が使われている
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/*
cargo test -- --nocapture 2>/dev/null
*/
#[test]
fn add_point() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}



//------------------------------------------------------------------------------
// ジェネリック型`RHS`についてデフォルト型（`Self`）を使わない例

// Listing 19-15: Implementing the Add trait on Millimeters to add Millimeters to Meters
struct Millimeters(u32);
struct Meters(u32);

//        ↓ デフォルトの`Self`ではなく、`Meters`を利用
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}



//------------------------------------------------------------------------------
// どのトレイトのflyメソッドを呼び出したいか指定する

// Listing 19-16: Two traits are defined to have a fly method and are implemented
//                on the Human type, and a fly method is implemented on Human directly
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// Listing 19-17: Calling fly on an instance of Human
// Listing 19-18: Specifying which trait’s fly method we want to call
fn calling_fly_on_an_instance_of_human() {
    let person = Human;
    person.fly(); // Listing 19-17
    Pilot::fly(&person); // Listing 19-18
    Wizard::fly(&person); // Listing 19-18
}



//------------------------------------------------------------------------------
//  関連関数がある型にその関連関数と同名の関連関数のあるトレイトを実装した場合 （Listing 19-19）

// Listing 19-19: A trait with an associated function and a type with an
//  associated function of the same name that also implements the trait
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot") // スポット(Wikipediaによると、飼い主の事故死後もその人の帰りを待つ忠犬の名前の模様)
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

/*
cargo test print_message -- --nocapture 2>/dev/null
*/
#[test]
fn print_message() {
    // Listing 19-20: Attempting to call the baby_name function from the Animal trait,
    //  but Rust doesn’t know which implementation to use
    println!("A baby dog is called a {}", Dog::baby_name());

    // Listing 19-21: Using fully qualified syntax to specify that we want to call
    //  the baby_name function from the Animal trait as implemented on Dog
    //                                     ↓ フルパス記法(fully qualified syntax)
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}



//------------------------------------------------------------------------------
// スーパートレイト

// Listing 19-22: Implementing the OutlinePrint trait that requires the functionality from Display
use std::fmt;

// trait OutlinePrint { // ← これでは`to_string`部分でエラーとなる
trait OutlinePrint: fmt::Display { // OutlinePrint は Displayトレイトを必要とすると指定
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl OutlinePrint for Point {} // ← impl fmt::Display for Point がないとエラーになる

/*
cargo test outline_print -- --nocapture 2>/dev/null
*/
#[test]
fn outline_print() {
    (Point { x: 100, y: 200 }).outline_print();
}



//------------------------------------------------------------------------------
// ニュータイプパターン
// （オーファンルールの回避方法）

// Listing 19-23: Creating a Wrapper type around Vec<String> to implement Display
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

/*
cargo test listing_19_23 -- --nocapture 2>/dev/null
*/
#[test]
fn listing_19_23() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}