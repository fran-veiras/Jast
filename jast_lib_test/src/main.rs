use jast_lib::{Http, Res, DataTypes, RouteResponse, Builder};

fn main() {
    fn controller() -> RouteResponse<'static> {
        let response = RouteResponse {
            method: "GET",
            res: Res::json(vec![
                ("name", DataTypes::Str(String::from("Juan"))), 
                ("lastname", DataTypes::Str(String::from("Alberto"))), 
                ("id", DataTypes::Int(i32::from(1)))
            ])
        };

        response
    }

    let routes = vec![
        Http::route("/", controller()),
        Http::route("/hola", 
            (|| RouteResponse { method: "GET", res: Res::json(vec![("name", DataTypes::Str("Juan".to_string()))]) })()),
        Http::route("/html", 
            (|| RouteResponse { method: "GET", res: "src/index.html"})())
    ];

    let settings = Builder::new("localhost:8080", routes).worker_threads(4);

    Http::create_server(
       settings
    )
}


