use std::thread;
use std::sync::{mpsc, Arc, Mutex};

// Listing 20-19: Creating a Job type alias for a Box that holds each closure
type Job = Box< dyn FnOnce() + Send + 'static>;

// Listing 20-17
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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
        assert!(size > 0);

        // Listing 20-17: Passing the receiving end of the channel to the workers
        let (sender, receiver) = mpsc::channel();

        // Listing 20-18: Sharing the receiving end of the channel among the workers using Arc and Mutex
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // workers.push(Worker::new(id, receiver)); // Listing 20-17
            workers.push(Worker::new(id, Arc::clone(&receiver))); // Listing 20-18
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // Listing 20-19: sending the job down the channel
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    // fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker { // Listing 20-17
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker { // Listing 20-18
        // Listing 20-20: Receiving and executing the jobs in the worker’s thread
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            job();
        });

        /* Listing 20-21: An alternative implementation of Worker::new using while let
        let thread = thread::spawn(move || {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {} got a job; executing.", id);

                job();
            }
        }); このコードだと、 MutexGuard の drop が job呼出し後も呼ばれないため、NG
            ∵ while let (and if let and match) does not drop temporary values until the end of the associated block.
        */
        Worker { id, thread }
    }
}
