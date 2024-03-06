use actix_web::{
    HttpServer,
    get,
    App,
    web::Path,
    Responder
};

use rhai::Engine;

#[get("/divide/{num1}/{num2}")]
async fn divide(path: Path<(i64, i64)>) -> impl Responder {
    let (num1, num2) = path.into_inner();

    let mut engine = Engine::new();
    engine.register_fn("num1", move || num1);
    engine.register_fn("num2", move || num2);

    let result = engine.eval_file::<i64>("src/divide.rhai".into()).unwrap();

    format!("{result}")
}

#[get("/multiply/{num1}/{num2}")]
async fn multiply(path: Path<(i64, i64)>) -> impl Responder {
    let (num1, num2) = path.into_inner();

    let mut engine = Engine::new();
    engine.register_fn("num1", move || num1);
    engine.register_fn("num2", move || num2);

    let result = engine.eval_file::<i64>("src/multiply.rhai".into()).unwrap();

    format!("{result}")
}

#[get("/add/{num1}/{num2}")]
async fn add(path: Path<(i64, i64)>) -> impl Responder {
    let (num1, num2) = path.into_inner();

    let mut engine = Engine::new();
    engine.register_fn("num1", move || num1);
    engine.register_fn("num2", move || num2);

    let result = engine.eval_file::<i64>("src/add.rhai".into()).unwrap();

    format!("{result}")
}

#[actix_web::main]
async fn main() ->std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(multiply)
        .service(add)
        .service(divide)
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
    .await
}

// cargo run and then open your browser
// type sth like: http://localhost:8080/multiply/5/3 and http://localhost:8080/add/5/3
// use ctrl + c to stop the program
// to kill a port you can use something like: sudo kill -9 `sudo lsof -t -i:8080`

// good source: https://blog.logrocket.com/building-rest-api-rust-rhai-actix-web/#simple-rust-rest-api-example-actix-web