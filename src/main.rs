use std::io;

fn main() {
    println!("Guess the number!"); // std::println(https://doc.rust-lang.org/std/macro.println.html)

    println!("Please input your guess.");

    let mut guess = String::new(); // std::string::String(https://doc.rust-lang.org/std/string/struct.String.html)/std::prelude(https://doc.rust-lang.org/std/prelude/index.html)

    io::stdin()                         // std::io::stdin(https://doc.rust-lang.org/std/io/fn.stdin.html)
        .read_line(&mut guess)          // https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line
        .expect("Failed to read line"); // std::io::Result（https://doc.rust-lang.org/std/io/type.Result.html）

    println!("You guessed: {}", guess);
}
