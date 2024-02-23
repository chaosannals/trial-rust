use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let port = 40404;
    log::info!("start hw at port: {:?}", port);

    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
