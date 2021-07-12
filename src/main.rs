use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!"); // https://doc.rust-lang.org/std/macro.println.html

    let secret_number = rand::thread_rng() // https://docs.rs/rand/0.8.4/rand/fn.thread_rng.html
                                .gen_range(1..101); // https://docs.rs/rand/0.8.4/rand/trait.Rng.html#method.gen_range

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new(); // https://doc.rust-lang.org/std/string/struct.String.html

    io::stdin()                         // https://doc.rust-lang.org/std/io/fn.stdin.html
        .read_line(&mut guess)          // https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line
        .expect("Failed to read line"); // https://doc.rust-lang.org/std/io/type.Result.html

    println!("You guessed: {}", guess);
}
