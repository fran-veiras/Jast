use crate::types::Routes;

#[derive(Debug)]
pub struct Builder<'a> {
    pub worker_threads: Option<usize>,
    pub routes: Vec<Routes<'static>>,
    pub host: &'a str
}


impl Builder<'_> {
    pub fn new<'a>(host: &'static str, routes: Vec<Routes<'static>>) -> Builder<'static> {
        Builder {
            worker_threads: None,
            routes,
            host
        }
    }    

    pub fn worker_threads(mut self, val: usize) -> Self {
        assert!(val > 0, "Worker threads cannot be set to 0");
        self.worker_threads = Some(val);
        self
    }
}
