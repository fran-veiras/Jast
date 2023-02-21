use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use serde_json::Value;

pub struct Http;
pub struct Res;

#[derive(Debug)]
#[derive(Clone)]



// cuerpo de las rutas
pub struct Routes<'a> {
    // GET / HTTP/1.1
    pub method: &'a str,
    pub route: &'a str,
    pub response: String,
}


// obtengo method, route y response x metodo llamado, falta hacer por route tb.
fn get_filtered_routes<'a>(routes: Vec<Routes<'a>>, parts_of_req: Vec<&str>) -> Vec<Routes<'a>> {
    let route = parts_of_req[1];
    let method = parts_of_req[0];

    // route 
    println!("route {}", route);

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
        if route.len() > 0 {
            let object: Value = serde_json::from_str(&route[0].response).unwrap();
        
            ("HTTP/1.1 200 OK", object)
        } else {
            let error_json = r#"{"error": "404 not found"}"#;
            let response: Value = serde_json::from_str(&error_json).unwrap();
            ("HTTP/1.1 404 NOT FOUND", response)
        }
    } else {
        let error_json = r#"{"error": "404 not found"}"#;
        let response: Value = serde_json::from_str(&error_json).unwrap();
        ("HTTP/1.1 404 NOT FOUND", response)
    };

    let contents = route_response;
    let length = contents.to_string().len();

    // armo cuerpo de la respuesta
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // respondo la request
    stream.write_all(response.as_bytes()).unwrap();
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
    pub fn json(res: Vec<(&str, Data)>) -> String {
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

        res_format_json.to_string()
    }
}


// res.setHeader("Content-Type", "text/html");
// res.writeHead(200);
// res.end(`<html><body><h1>This is HTML</h1></body></html>`);

