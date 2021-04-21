use std::thread;
use std::sync::{self, Arc, Mutex };


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: sync::mpsc::Sender<Job>
}
struct Worker {
    id: usize,
    thread_handle: thread::JoinHandle<Arc<Mutex<sync::mpsc::Receiver<Job>>>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;



impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<sync::mpsc::Receiver<Job>>>) -> Worker {
        Worker {
            id,
            thread_handle: thread::spawn( move || {
                loop {
                    let job = receiver.lock().unwrap().recv().unwrap();
                    println!("Worker {} got a job; executing.", id);
                    job()
                }
            }),
        }
    }
}

impl ThreadPool {
    /// Creates  a new threadpoolthreads to create
    ///
    /// Takes as sole argument the number of
    /// #Panics
    ///
    /// if number of pools to create
    /// passed is less than 1
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = sync::mpsc::channel();
        let receiver: Arc<Mutex<sync::mpsc::Receiver<Job>>> = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}
