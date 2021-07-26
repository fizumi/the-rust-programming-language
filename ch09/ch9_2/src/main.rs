
// ch 9.2. last
/* The main function is special, and there are restrictions on
what its return type must be. One valid return type for main is (),
and conveniently, another valid return type is Result<T, E>, as shown here:*/
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    use std::fs::File;
    let f = File::open("hello.txt")?;

    Ok(())
}

// Listing 9-5: Handling different kinds of errors in different ways
// The match expression is very useful but also very much a primitive.
fn listing_9_5() {
    use std::fs::File;
    use std::io::ErrorKind;
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

// the `Result<T, E>` type has many methods that accept a closure
// and are implemented using match expressions. Using those methods will make
// your code more concise.
fn listing_9_5_improved() {
    use std::fs::File;
    use std::io::ErrorKind;
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
// Although this code has the same behavior as Listing 9-5,
// it doesn’t contain any match expressions and is cleaner to read.


// -----------------------------------------------------------------------------
// Shortcuts for Panic on Error

fn unwrap() {
    use std::fs::File;
    let f = File::open("hello.txt").unwrap();
}

fn expect() {
    use std::fs::File;
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}


// -----------------------------------------------------------------------------
// Propagating Errors

// Listing 9-6: A function that returns errors to the calling code using match
fn listing_9_6() {
    use std::io;
    use std::io::Read;
    use std::fs::File;

    // `?`演算子なしの Propagating Errors
    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
}

// Listing 9-7: A function that returns errors to the calling code
//              using the ? operator
fn listing_9_7() {
    use std::io;
    use std::io::Read;
    use std::fs::File;

    // `?`演算子ありの Propagating Errors
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
}


// Listing 9-8: Chaining method calls after the ? operator
fn listing_9_8() {
    use std::io;
    use std::io::Read;
    use std::fs::File;

    // `?`演算子ありの Propagating Errors （+ メソッドチェーン（Chaining method））
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }
}

