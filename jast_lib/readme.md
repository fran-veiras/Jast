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
    <img alt="" src="https://github.com/fran-veiras/Jast/assets/74626997/41c97e54-6226-4818-83df-d86baee22251" height="28">
  </a>
</p>

<p>Jast is an open source minimalist Rust library for building high-performance web servers. Designed to be both easy to use and performant, Jast takes advantage of Rust's strengths in areas such as security and memory usage.</p>

[Jast](https://crates.io/crates/jast) (versión: 0.1.2)

## Install
<h5 a><strong><code>Cargo.toml</code></strong></h5>

``` toml
[dependencies]
jast = "0.1.2"
```

## Setup 

#### Basic server 
``` rust
fn controller() -> RouteResponse<'static> {
        let response = RouteResponse {
            method: "GET",
            res: Res::json(vec![
                ("name", DataTypes::Str(String::from("Hello"))), 
                ("lastname", DataTypes::Str(String::from("World"))), 
                ("id", DataTypes::Int(i32::from(1)))
            ])
        };

        response
}

let routes = vec![
        Http::route("/", controller())
];

let settings = Builder::new("localhost:8080", routes);

Http::create_server(
   settings
)
```

#### Server settings
<p>By default it uses the number of cpu cores to determine the number of worker threads. This can be changed as follows:</p>

``` rust
let settings = Builder::new("localhost:8080", routes).worker_threads(4);
```

## Usage

#### new route
<p>Struct of Routes to create new routes</p>

``` rust
pub struct Routes<'a> {
    pub method: &'a str, 
    pub route: &'a str,
    pub response: &'a str,
}
```

``` rust
Http::route(method, route, response)
```

#### response
<p>Json:</p>

``` rust
Res::json(vec![((name, value)))])
```

<p>struct of DataTypes to value</p>

``` rust
Res::json(vec![("name", DataTypes::Str(String::from("Hello")))])
```

<p>File: from the root of the project</p>

```rust
Http::route("/html", (|| RouteResponse { method: "GET", res: "src/index.html"})()) 
```

## Example

``` rust
use jast::{Http, Res, DataTypes, RouteResponse, Builder};

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

## Contribute
