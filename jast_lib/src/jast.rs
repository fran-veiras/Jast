mod utils;

mod routes;
pub use routes::types;

mod connection;
pub use connection::http::Http;
pub use connection::response;

pub use connection::response::Res;
pub use connection::types::DataTypes;



#[derive(Debug)]
pub struct RouteResponse<'a> {
   pub method: &'a str, 
   pub res: &'a str
}


