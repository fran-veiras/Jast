use jast_lib::jast::Http;
use jast_lib::jast::Res;
use jast_lib::jast::Routes;
use jast_lib::jast::Data;

// http.get

fn main() {
    println!("inside main of test ");

    let route1 = Routes {
        method: "GET",
        route: "/hola",
        response: Res::json(vec![
            ("name", Data::Str(String::from("Francisco"))), 
            ("lastname", Data::Str(String::from("Veiras"))), 
        ])
    };

    let route2 = Routes {
        method: "GET",
        route: "/chau",
        response: Res::json(vec![
            ("name", Data::Str(String::from("Juan"))), 
            ("lastname", Data::Str(String::from("Alberto"))), 
            ("id", Data::Int(i32::from(1)))
        ])
    };




    let routes = vec![route1, route2];

    Http::create_server(
        "localhost", 
        "8080", 
        routes
    )
}
