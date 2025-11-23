mod worker;

use std::sync::{Arc, Mutex, mpsc};

use self::worker::*;

/// ThreadPool used to parallelize workload to be more efficient and responsive
///
/// ## Example
/// ```
/// use std::{
///    io::{prelude::*, BufReader},
///    net::{TcpListener, TcpStream},
///    time::Duration,
///};
///
///use hello::thread_pool::ThreadPool;
///
///fn main() {
///    let addr_listen = "127.0.0.1:7878";
///    let pool = ThreadPool::new(4);
///
///    let listener = TcpListener::bind(addr_listen).unwrap();
///
///    println!("Listening to {}", addr_listen);
///
///    for stream in listener.incoming().take(2) {
///        let stream = stream.unwrap();
///        pool.execute(|| handle_connection(stream));
///    }
///
///    println!("Shutting down.");
///}
/// ```
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The 'new' function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    ///Execute the code in a separate thread
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            log::debug!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
