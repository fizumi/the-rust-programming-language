use std::thread;
use std::time::Duration;

// スレッドを使用してコードを同時に走らせる
fn main() {
    use_spawn();
    use_move();
}

// Listing 16-1: Creating a new thread to print one thing while the main thread prints something else
// Listing 16-2: Saving a JoinHandle from thread::spawn to guarantee the thread is run to completion
fn use_spawn() {

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap(); // Listing 16-2

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // Listing 16-2
}

// スレッドでmoveクロージャを使用する
// Listing 16-5: Using the move keyword to force a closure to take ownership of the values it uses
fn use_move() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
    // println!("{:?}", v); // ❌ already moved
}