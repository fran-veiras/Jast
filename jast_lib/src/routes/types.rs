#[derive(Debug)]
#[derive(Clone)]

pub struct Routes<'a> {
    // GET / HTTP/1.1
    pub method: &'a str,
    pub route: &'a str,
    pub response: &'a str,
}
