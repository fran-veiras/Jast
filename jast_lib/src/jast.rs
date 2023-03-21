use serde_json::Value;
mod utils;

mod routes;
use routes::types;

mod connection;
pub use connection::http::Http;

pub struct Res;


#[derive(Debug)]
pub struct RouteResponse<'a> {
   pub method: &'a str, 
   pub res: &'a str
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

