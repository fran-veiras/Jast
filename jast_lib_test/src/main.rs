
use jast_lib::jast::{Http, Res, Data, RouteResponse};

fn main() {
    println!("inside main of test ");

    fn controller() -> RouteResponse<'static> {
        let response = RouteResponse {
            method: "GET",
            res: Res::json(vec![
                ("name", Data::Str(String::from("Juan"))), 
                ("lastname", Data::Str(String::from("Alberto"))), 
                ("id", Data::Int(i32::from(1)))
            ])
        };

        response
    }

    let routes = vec![
        Http::route("/", controller()),
        Http::route("/hola", 
            (|| RouteResponse { method: "GET", res: Res::json(vec![("name", Data::Str("Juan".to_string()))]) })()),
        Http::route("/html", 
            (|| RouteResponse { method: "GET", res: "src"})())
    ];

    Http::create_server(
        "localhost", 
        "8080", 
        routes
    )
}


