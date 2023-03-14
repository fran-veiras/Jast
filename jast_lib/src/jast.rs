use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use std::fs::OpenOptions;
use std::fs;
use serde_json::{json, Value};
use std::env;

pub struct Http;
pub struct Res;
#[derive(Debug)]
#[derive(Clone)]

// cuerpo de las rutas
pub struct Routes<'a> {
    // GET / HTTP/1.1
    pub method: &'a str,
    pub route: &'a str,
    pub response: &'a str,
}

fn error_log(err_message : &str) -> std::io::Result<()> {
    match fs::File::open("jast_err_logs.txt") {
        Ok(mut file) => {
            // implementar escribir errores 
            writeln!(file, "{};", err_message);

            Ok(())
        },
        Err(err) => {
            let mut current_dir = env::current_dir().unwrap();
            current_dir.push("jast_err_logs.txt");

            fs::File::create(current_dir);

            Ok(())
        }
    }
}

// obtengo method, route y response x metodo llamado, falta hacer por route tb.
fn get_filtered_routes<'a>(routes: Vec<Routes<'a>>, parts_of_req: Vec<&str>) -> Vec<Routes<'a>> {
    let route = parts_of_req[1];
    let method = parts_of_req[0];

    // route 
    let filtered_routes: Vec<Routes<'_>> = routes
        .into_iter()
        .filter(|voc| voc.method == method.to_string())
        .filter(|voc| voc.route == route.to_string())
        .collect();

    return filtered_routes;
}

fn handle_connection(mut stream: TcpStream, routes: Vec<Routes<'_>>) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let parts_of_req: Vec<&str> = request_line.split(' ').collect();

    // status_line como response a cada metodo con la respuesta (route_response) json o texto
    let (status_line, route_response) = if request_line.contains("GET") {
        let route = get_filtered_routes(routes, parts_of_req);

        let error_json = r#"{"error": "404 not found"}"#;
        // pasarlo a fn para diferentes error codes
        let response_err_json: Value = serde_json::from_str(&error_json).unwrap();
        
        if route.len() > 0 {

            let result = serde_json::from_str(&route[0].response);
            let hola = result.unwrap_or_default();

            let mess = match hola {
                Some(object) => ("HTTP/1.1 200 OK", object),
                None => ("HTTP/1.1 200 OK", json!({"other": &route[0].response})),
            };

            mess
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
            error_log("hola");

            // current dir usa como root la ruta del proyecto en la que esta instalado.
            let mut current_dir = env::current_dir().unwrap();
            // le pusheo el nombre del archivo
            current_dir.push(file_name);
            
            match fs::read_to_string(current_dir) {
                Ok(content) => {
                    let length = content.len();
            
                    let response = format!(
                        "{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
            
                    stream.write_all(response.as_bytes()).unwrap();  
                },
                Err(_) => {

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

#[derive(Debug)]
pub struct RouteResponse<'a> {
   pub method: &'a str, 
   pub res: &'a str
}


impl Http {   
    // impementación del server
    pub fn create_server(host: &str, port: &str, routes: Vec<Routes<'_>>) {
        let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            
            handle_connection(stream, routes.clone());
        }
    }


    // generar nuevas rutas
    pub fn route<'a>(route_name: &'static str, response: RouteResponse<'static>) -> Routes<'a> {
        let new_route = Routes {
            method: response.method,
            route: route_name,
            response: response.res 
        };

        new_route
    }
}



struct DynamicStruct {
    fields: Vec<(String, Value)>,
}

#[derive(Debug)]
pub enum Data {
    Str(String),
    Int(i32)
}

impl Res {
    // response en formato json
    pub fn json(res: Vec<(&str, Data)>) -> &'static str {
        let mut dynamic_struct = DynamicStruct { fields: vec![] };

        for (response, data) in res {
            match data {
                Data::Str(s) => dynamic_struct.fields.push((format!("{}", response), Value::String(format!("{}", s)))),
                Data::Int(num) => dynamic_struct.fields.push((format!("{}", response), Value::Number( serde_json::Number::from(num) ))),
            }
        }

        let mut input_to_json = String::new();

        for (index, value) in dynamic_struct.fields.iter().enumerate() {

            // es el último elemento ? mando sin coma (para no romper el json) : coma final
            if index == dynamic_struct.fields.len() - 1 {
                let json_format = format!("\"{}\":{}", value.0, value.1);
                input_to_json.push_str(&json_format)
            } else {
                let json_format = format!("\"{}\":{},", value.0, value.1);
                input_to_json.push_str(&json_format)
            }
        };


        // formato para pasar a json en handle response
        let res_format_json = format!(r#"{{ {} }}"#, input_to_json);

        Box::leak(res_format_json.into_boxed_str())
    }
}


// res.setHeader("Content-Type", "text/html");
// res.writeHead(200);
// res.end(`<html><body><h1>This is HTML</h1></body></html>`);

