use std::thread;
use std::sync::mpsc; // ※ mpsc was an acronym for multiple producer, single consumer
use std::time::Duration;

// メッセージ受け渡しを使ってスレッド間でデータを転送する
#[test]
fn listing_16_7_to_16_9() {
    let (tx, rx) = mpsc::channel(); // Listing 16-6: Creating a channel and assigning the two halves to tx and rx

    // Listing 16-7: Moving tx to a spawned thread and sending “hi”
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val); // borrow of moved value (E0382) // Listing 16-9: Attempting to use val after we’ve sent it down the channel
    });

    // Listing 16-8: Receiving the value “hi” in the main thread and printing it
    let received = rx.recv().unwrap(); // recv: メインスレッドの実行をブロックし、 値がチャンネルを流れてくるまで待機
    println!("Got: {}", received);
} // cargo test listing_16_7_to_16_9 -- --nocapture


// 複数の値を送信し、受信側が待機するのを確かめる
// Listing 16-10: Sending multiple messages and pausing between each
#[test]
fn listing_16_10() {
    let (tx, rx) = mpsc::channel();

    // Listing 16-10: Sending multiple messages ..
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Listing 16-10: .. and pausing between each
    for received in rx {
        println!("Got: {}", received);
    }
} // cargo test listing_16_10 -- --nocapture


// 転送機をクローンして複数の生成器を作成する
// Listing 16-11: Sending multiple messages from multiple producers
fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

} // cargo run
