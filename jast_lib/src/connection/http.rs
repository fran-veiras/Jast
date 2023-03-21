use std::net::TcpListener;
use crate::RouteResponse;
use crate::connection::handle_connection::handle_connection;
use crate::routes::types;

pub struct Http;

impl Http {   
    // impementación del server
    pub fn create_server(host: &str, port: &str, routes: Vec<types::Routes<'_>>) {
        let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            
            handle_connection(stream, routes.clone());
        }
    }


    // generar nuevas rutas
    pub fn route<'a>(route_name: &'static str, response: RouteResponse<'static>) -> types::Routes<'a> {
        let new_route = types::Routes {
            method: response.method,
            route: route_name,
            response: response.res 
        };

        new_route
    }
}
