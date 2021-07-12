use std::io;
use std::cmp::Ordering;
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

    let guess: u32 = guess // shadow
        .trim() // https://doc.rust-lang.org/std/string/struct.String.html#method.trim（末尾の「改行」を削除）
        .parse() // https://doc.rust-lang.org/std/primitive.str.html#method.parse
        .expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) { // https://doc.rust-lang.org/std/cmp/trait.Ord.html#tymethod.cmp
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
