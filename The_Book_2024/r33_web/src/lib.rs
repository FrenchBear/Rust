// r33_web
// Learning rust 2024, Multithreaded Web Server
//
// 2025-03-16   PV

pub struct ThreadPool;

impl ThreadPool {
    /// Create a new ThreadPool.
    /// The size is the number of threads in the pool.
    /// # Panics
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        ThreadPool
    }

    // --snip--
    pub fn execute<F>(&self, _f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
