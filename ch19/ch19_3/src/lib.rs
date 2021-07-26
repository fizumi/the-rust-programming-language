// -----------------------------------------------------------------------------
// type alias
// -----------------------------------------------------------------------------
// Kilometers と i32 は同じ型
#[test]
fn type_alias() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

// ❌ 記述量の多い型（Box<Fn() + Send + 'static>）が繰り返り利用されている
// Listing 19-24: Using a long type in many places
fn listing_19_24() {
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --snip--
    }

    fn returns_long_type(a: (), f: Box<dyn Fn() + Send + 'static>) {
        // --snip--
    }
}

// ⭕ 冗長な記載（Box<Fn() + Send + 'static>）をシンプル（Thunk）にする
// Listing 19-25: Introducing a type alias Thunk to reduce repetition
fn listing_19_25() {
    type Thunk = Box<dyn Fn() + Send + 'static>; // ✔ type alias

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type(a: (), f: Thunk) {
        // --snip--
    }
}

// ❌ Result<..., Error>が何度も繰り返されている
mod case1 {
use std::io::Error;
use std::fmt;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}
}
// ⭕ 型エイリアスにより、コードを書きやすくし、
//    一貫したインターフェイス（エラーの型は一貫して`std::io::Error'）を与える
mod case2{
use std::fmt;

type Result<T> = std::result::Result<T, std::io::Error>; // ✔ type alias
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
}

// -----------------------------------------------------------------------------
// never型 / never type （ Empty型 / empty type）
// -----------------------------------------------------------------------------
fn part_of_guessing_game() {
    loop {
        let mut guess = String::new();
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // u32 型でないが、never 型なので許容される
        };
    }
}

enum MyOption<T> {
    Some(T),
    None
}
impl<T> MyOption<T> {
    pub fn unwrap(self) -> T {
        match self {
            MyOption::Some(val) => val,
            MyOption::None => panic!("called `Option::unwrap()` on a `None` value"), // T 型でないが、never 型なので許容される
        }
    }
}

// `!` は never 型 を表す
fn loop_forever() -> ! {
    loop {
        print!("and ever ");
    }
}