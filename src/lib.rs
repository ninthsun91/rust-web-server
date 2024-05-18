use std::{fmt, process::id, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// size: The number of threads in the pool.
    /// 
    /// # Panics
    /// - `new` will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let workers = create_workers(size);

        ThreadPool { workers }
    }

    /// Create a new ThreadPool.
    /// 
    /// size: The number of threads in the pool. If the size is zero, return PoolCreationError.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        let workers = create_workers(size);

        if size > 0 {
            Ok(ThreadPool { workers })
        } else {
            Err(PoolCreationError)
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {}
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}

fn create_workers(size: usize) -> Vec<Worker> {
    let mut workers = Vec::with_capacity(size);
    for id in 0..size {
        workers.push(Worker::new(id));
    }

    workers
}
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PoolCreationError")
    }
}