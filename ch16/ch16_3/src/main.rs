use std::sync::{Mutex, Arc};
use std::thread;


// Listing 16-12: Exploring the API of Mutex<T> in a single-threaded context for simplicity
#[test]
fn listing_16_12() {
    let m = Mutex::new(5);
    println!("m = {:?}", m); // data: 5

    {
        let mut num = m.lock().unwrap();
        println!("m = {:?}", m); // data: <locked>
        println!("num = {}", num); // 5 （num は `MutexGuard` というスマートポインタ）
        *num = 6;
        println!("num = {}", num); // 6 （num は `MutexGuard` というスマートポインタ）

        // let num2 = m.lock(); // lock 解除待ちになる
        // println!("num2 = {:?}", num2);
    } // num が drop され、 lock が解除される

    println!("m = {:?}", m); // data: 6
} // cargo test listing_16_12 -- --nocapture

// Listing 16-13: Ten threads each increment a counter guarded by a Mutex<T>
// Listing 16-14: Attempting to use Rc<T> to allow multiple threads to own the Mutex<T>
// Listing 16-15: Using an Arc<T> to wrap the Mutex<T> to be able to share ownership across multiple threads
fn main() {
    // let counter = Mutex::new(0); // Listing 16-13 ❌ counter の所有権を共有できない
    // let counter = Rc::new(Mutex::new(0)); // Listing 16-14 ❌ Rc<T>はスレッド間で共有するには安全ではない
    let counter = Arc::new(Mutex::new(0)); // ⭕
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // ⭕ counter の所有権をスレッド間で安全に共有
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}