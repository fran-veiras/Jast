use crate::types::Routes;

#[derive(Debug)]
/// Builder struct for configuring a server.
pub struct Builder<'a> {
    /// Number of worker threads for the server.
    pub worker_threads: Option<usize>,
    
    /// List of routes for the server.
    pub routes: Vec<Routes<'static>>,
    
    /// Host address for the server.
    pub host: &'a str
}

impl Builder<'_> {
    /// Basic server settings.
    ///
    /// # Arguments
    ///
    /// * host - local server address.
    /// * routes - server routes.
    ///
    /// # Returns
    ///
    /// Struct of Builder (worker_threads (if not defined, the number of cpus available), routes, host)
    pub fn new<'a>(host: &'static str, routes: Vec<Routes<'static>>) -> Builder<'static> {
        Builder {
            worker_threads: None,
            routes,
            host
        }
    }    

    /// Custom number of workers.
    ///
    /// # Arguments
    ///
    /// * self - Builder struct.
    /// * workers - number of workers.
    ///
    /// # Returns
    ///
    /// Struct of Builder (worker_threads, routes, host)
    pub fn worker_threads(mut self, workers: usize) -> Self {
        assert!(workers > 0, "Worker threads cannot be set to 0");
        self.worker_threads = Some(workers);
        self
    }
}
