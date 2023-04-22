
use std::net::TcpListener;
use crate::RouteResponse;
use crate::connection::handle_connection::handle_connection;
use crate::routes::types;
use crate::connection::threadpool::ThreadPool;
use std::sync::Arc;
pub struct Http;
use crate::Builder;

use num_cpus;

impl Http {
    pub fn create_server(runtime : &mut Builder<'static>) {
        let listener = TcpListener::bind(runtime.host).unwrap();
        let routes = Arc::new(runtime.routes.clone());

        let default_threads = num_cpus::get_physical();

        let pool = ThreadPool::new(default_threads);

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
