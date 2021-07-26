// レスポンスを記述する
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

const HELLO_HTML_PATH: &str = "../../hello.html";
const NOT_FOUND_HTML_PATH: &str = "../../404.html";


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // handle_connection_20_2(stream);
        // handle_connection_20_3(stream);
        // handle_connection_20_5(stream);
        // handle_connection_20_7(stream);
        handle_connection(stream);
    }
}

// リクエストを読み取る
// Listing 20-2: Reading from the TcpStream and printing the data
//                    ↓ TcpStreamインスタンスは内部で返すデータを追いかけているため、性質上可変
fn handle_connection_20_2(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}


// Listing 20-3: Writing a tiny successful HTTP response to the stream
fn handle_connection_20_3(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    // HTTPバージョン1.1を使用、ステータスコードが200、OKフレーズ、ヘッダと本体なしのレスポンス
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();

    //      ↓ バイトが全て接続に書き込まれるまで待機する
    stream.flush().unwrap();
}

// 本物のHTMLを返す
// Listing 20-5: Sending the contents of hello.html as the body of the response
use std::fs;
fn handle_connection_20_5(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string(HELLO_HTML_PATH).unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// リクエストにバリデーションをかけ、選択的にレスポンスを返す
// Listing 20-6: Matching the request and handling requests to / differently from other requests
fn handle_connection_20_7(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {                                                // https://doc.rust-lang.org/std/primitive.slice.html#method.starts_with
        let contents = fs::read_to_string(HELLO_HTML_PATH).unwrap();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        // Listing 20-7: Responding with status code 404 and an error page if anything other than / was requested
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string(NOT_FOUND_HTML_PATH).unwrap();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

// 少しリファクタリング
// Listing 20-9: Refactoring the if and else blocks to contain only the code that differs between the two cases
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", HELLO_HTML_PATH)
    } else {
        ("HTTP/1.1 404 NOT FOUND", NOT_FOUND_HTML_PATH)
    };

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
