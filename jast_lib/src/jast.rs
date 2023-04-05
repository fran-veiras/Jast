mod utils;

mod routes;
use routes::types;

mod connection;
pub use connection::http::Http;
pub use connection::response;


#[derive(Debug)]
pub struct RouteResponse<'a> {
   pub method: &'a str, 
   pub res: &'a str
}


