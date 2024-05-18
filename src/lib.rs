use std::{fmt, sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// size: The number of threads in the pool.
    /// 
    /// # Panics
    /// - `new` will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (workers, sender) = create_workers(size);

        ThreadPool { workers, sender }
    }

    /// Create a new ThreadPool.
    /// 
    /// size: The number of threads in the pool. If the size is zero, return PoolCreationError.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        let (workers, sender) = create_workers(size);

        if size > 0 {
            Ok(ThreadPool { workers, sender })
        } else {
            Err(PoolCreationError)
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // `thread::spawn` will panic if the os doesn't have enough resources to create a new thread.
        // Therefore, use std::thread::Builder to create a new thread in the real word.
        let thread = thread::spawn(move || loop {
            println!("Worker {} got a job; executing.", id);
            let job = receiver.lock().unwrap().recv().unwrap();

            job();
        });

        Worker { id, thread }
    }
}

fn create_workers(size: usize) -> (Vec<Worker>, mpsc::Sender<Job>) {
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut workers = Vec::with_capacity(size);
    for id in 0..size {
        workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    (workers, sender)
}
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PoolCreationError")
    }
}