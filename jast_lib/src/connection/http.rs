
use std::net::TcpListener;
use crate::RouteResponse;
use crate::connection::handle_connection::handle_connection;
use crate::routes::types;
use crate::connection::threadpool::ThreadPool;
use std::sync::Arc;

pub struct Http;

impl Http {
    pub fn create_server(host: &str, port: &str, routes: Vec<types::Routes<'static>>) {
        let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();
        let routes = Arc::new(routes);
        let pool = ThreadPool::new(4);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let all_routes = Arc::clone(&routes);

            pool.execute(move|| {
                handle_connection(stream, &all_routes);
            });
        }
    }

    pub fn route<'a>(route_name: &'static str, response: RouteResponse<'static>) -> types::Routes<'a> {
        let new_route = types::Routes {
            method: response.method,
            route: route_name,
            response: response.res
        };

        new_route
    }
}
