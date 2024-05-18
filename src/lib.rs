use std::fmt;

pub struct ThreadPool;

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// size: The number of threads in the pool.
    /// 
    /// # Panics
    /// - `new` will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        ThreadPool
    }

    /// Create a new ThreadPool.
    /// 
    /// size: The number of threads in the pool. If the size is zero, return PoolCreationError.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            Ok(ThreadPool)
        } else {
            Err(PoolCreationError)
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {}
}

pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PoolCreationError")
    }
}