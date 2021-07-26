use std::net::TcpListener;
use hello2::ThreadPool;

/*
fn making_new_threads_without_any_limit() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            hello::handle_connection(stream);
        });
    }
}*/
// ↑ の標準ライブラリ thread::spawn 実装に似たインターフェースを持つように ThreadPool を実装
// Listing 20-12: Our ideal ThreadPool interface
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            hello::handle_connection(stream);
        });
    }
}
