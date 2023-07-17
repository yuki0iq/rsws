pub struct ThreadPool;

impl ThreadPool {
    /// Constructs a new ThreadPool with given number of threads inside.
    ///
    /// # Panics
    ///
    /// Will panic if desired thread count is zero 
    // TODO: instead of panicing, return result
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        //
    }
}
