use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    ///Create a new Worker to run a functions on a separate thread
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        log::debug!("Worker {id} got a job; executing.");

                        job();
                    }
                    Err(_) => {
                        log::debug!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
