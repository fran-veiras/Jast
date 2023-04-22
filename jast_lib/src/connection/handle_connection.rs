use std::net::TcpStream;
use std::io::{prelude::*, BufReader};
use serde_json::{json, Value};
use std::{env, fs};

use crate::routes::{types, route_handler};
use crate::utils::error_log;

pub fn handle_connection(mut stream: TcpStream, routes: &Vec<types::Routes>) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let parts_of_req: Vec<&str> = request_line.split(' ').collect();


    // status_line como response a cada metodo con la respuesta (route_response) json o texto
    let (status_line, route_response) = if request_line.contains("GET") {
        let route = route_handler::get_filtered_routes(&routes, parts_of_req);

        let error_json = r#"{"error": "404 not found"}"#;
        // pasarlo a fn para diferentes error codes
        let response_err_json: Value = serde_json::from_str(&error_json).unwrap();
        
        if route.len() > 0 {
            let result = serde_json::from_str(&route[0].response);
            let json = result.unwrap_or_default();

            let response = match json {
                Some(object) => ("HTTP/1.1 200 OK", object),
                None => ("HTTP/1.1 200 OK", json!({"other": &route[0].response})),
            };

            response
        } else {
            ("HTTP/1.1 404 NOT FOUND", response_err_json)
        }
    } else {
        let error_json = r#"{"error": "404 not found"}"#;
        let response: Value = serde_json::from_str(&error_json).unwrap();
        ("HTTP/1.1 404 NOT FOUND", response)
    };

    // TODO: refactor -> soporte de text
    // si el campo other existe lo leemos como string
    if let Some(name_value) = route_response.get("other") {
        if let Some(file_name) = name_value.as_str() {
            // current dir usa como root la ruta del proyecto en la que esta instalado.
            let mut current_dir = match env::current_dir() {
                Ok(dir) => dir,
                Err(e) => {
                    error_log(&format!("{}", e)).unwrap();

                    panic!("{}", e);                    
                }
            };
            // le pusheo el nombre del archivo
            current_dir.push(file_name);
            
            match fs::read_to_string(current_dir) {
                Ok(content) => {
                    let length = content.len();
            
                    let response = format!(
                        "{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
            
                    stream.write_all(response.as_bytes()).unwrap();  
                },
                Err(e) => {
                    error_log(&format!("{}", e)).unwrap();

                    panic!("{}", e);                    
                }
            }

        }
    } else {
        let contents = route_response;
        let length = contents.to_string().len();
        
        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}
