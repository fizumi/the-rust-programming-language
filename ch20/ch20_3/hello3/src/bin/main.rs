use std::net::TcpListener;
use hello3::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // for stream in listener.incoming() {
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            hello::handle_connection(stream);
        });
    }

    println!("Shutting down.");
}
