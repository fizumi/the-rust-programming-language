// TCP接続をリッスンする

// Listing 20-1: Listening for incoming streams and printing a message when we receive a stream
use std::net::TcpListener;

fn main() {
    //                          ↓ ネットワークにおいて、リッスンすべきポートに接続することは、 binding to a port と表現される
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    //                      ↓ 実際には接続の試行を繰り返している
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
