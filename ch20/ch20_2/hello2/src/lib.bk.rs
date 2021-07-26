// ※ main.rs を src/bin/ に移動させることで、lib.rs が hello ディレクトリ内で主要クレートとなる
use std::thread;

pub struct ThreadPool {
    // Listing 20-14
    // threads: Vec<thread::JoinHandle<()>>,

    // Listing 20-15
    threads: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        // Listing 20-13: Implementing ThreadPool::new to panic if size is zero
        assert!(size > 0);

        // Listing 20-14: Creating a vector for ThreadPool to hold the threads
        let mut threads = Vec::with_capacity(size); // slightly more efficient than using `Vec::new`

        // Listing 20-15
        for id in 0..size {
            threads.push(Worker::new(id));
        }

        ThreadPool { threads }
    }

    // c.f. https://doc.rust-lang.org/std/thread/fn.spawn.html
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {


    }
}

// `Worker` はプール実装では一般的な用語
// Listing 20-15: Modifying ThreadPool to hold Worker instances instead of holding threads directly
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize) -> Worker {
        Worker { id, thread: thread::spawn(|| ()) }
    }
}