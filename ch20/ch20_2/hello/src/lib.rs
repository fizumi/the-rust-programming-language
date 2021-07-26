

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use std::thread;
use std::time::Duration;

const HELLO_HTML_PATH: &str = "../../hello.html";
const NOT_FOUND_HTML_PATH: &str = "../../404.html";

pub fn simple_listener() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

pub fn making_new_threads_without_any_limit() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // ❌ 無制限にスレッドを大量生産する可能性がある
        // Listing 20-11: Spawning a new thread for each stream
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // Listing 20-10: Simulating a slow request by recognizing /sleep and sleeping for 5 seconds
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", HELLO_HTML_PATH)
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", HELLO_HTML_PATH)
    } else {
        ("HTTP/1.1 404 NOT FOUND", NOT_FOUND_HTML_PATH)
    };
    // Listing 20-10: END

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
