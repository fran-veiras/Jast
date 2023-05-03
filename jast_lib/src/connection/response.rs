
use serde_json::Value;
use crate::connection::types::DataTypes;
pub struct Res;

struct DynamicStruct {
    fields: Vec<(String, Value)>,
}

impl Res {
    /// Json response.
    ///
    /// # Arguments
    ///
    /// * res - Vector of (field name (&str) & struct of DataTypes) 
    ///
    /// # Returns
    ///
    /// str formatted for json.
    pub fn json(res: Vec<(&str, DataTypes)>) -> &'static str {
        let mut dynamic_struct = DynamicStruct { fields: vec![] };

        for (response, data) in res {
            // dos posibles tipos de dato en el valor de una json key
            match data {
                DataTypes::Str(s) => dynamic_struct.fields.push((format!("{}", response), Value::String(format!("{}", s)))),
                DataTypes::Int(num) => dynamic_struct.fields.push((format!("{}", response), Value::Number( serde_json::Number::from(num) ))),
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
