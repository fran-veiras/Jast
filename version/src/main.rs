use std::{fs, env};
use toml::Value;

fn main() {
    let cargo_toml = fs::read_to_string("../jast_lib/Cargo.toml").expect("Failed to parse the Cargo.toml file");
    let mut toml: Value = cargo_toml.parse().expect("Failed to parse the Cargo.toml file");

    let version = toml["package"]["version"].as_str().expect("Version not found in Cargo.toml");

    let version_type : String = env::var("CARGO_RUN_VERSION_TYPE").unwrap();

    let version_parts: Vec<u32> = version
        .split('.')
        .map(|part| part.parse().unwrap())
        .enumerate()
        .map(|(pos, mut num)| {
            match version_type.as_str() {
                "major" => if pos == 0 {num += 1},
                "minor" => if pos == 1 {num += 1},
                "patch" => if pos == 2 {num += 1},
                _ => println!("version err"),
            };

            num
        })
        .collect();

    let new_version : String = version_parts.iter().map(|num| num.to_string()).collect::<Vec<String>>().join(".");

    toml["package"]["version"] = Value::String(new_version.to_owned());
        
    let updated_toml = toml.to_string();

    fs::write("../jast_lib/Cargo.toml", updated_toml).expect("Failed to write Cargo.toml file");

    return 
}
