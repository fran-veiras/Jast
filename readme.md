<p align="center">
  <a href="https://jast-docs.vercel.app/">
    <picture>
      <img src="https://user-images.githubusercontent.com/74626997/236079356-0a617dae-119e-4c54-8f32-0ac0ab2bbcc3.png" height="128">
    </picture>
    <h1 align="center">Jast</h1>
  </a>
</p>

<p align="center">
  <a aria-label="NPM version" href="https://crates.io/crates/jast_lib">
    <img alt="" src="https://user-images.githubusercontent.com/74626997/236079964-50ca76e7-5006-4477-97cf-35110fdc595c.png" height="28">
  </a>
</p>

### Install
<h5 a><strong><code>Cargo.toml</code></strong></h5>

``` toml
[dependencies]
jast_lib = "0.1.1"
```

### Example

``` rust
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

```
