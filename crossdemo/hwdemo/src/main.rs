use actix_web::{get, web, App, HttpServer, Responder};
use std::sync::Mutex;

mod app;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let counter = web::Data::new(app::hello::AppStateWithCounter {
        counter: Mutex::new(0),
    });

    let port = 40404;
    log::info!("start hw at port: {:?}", port);

    HttpServer::new(move || {
        App::new()
            .service(greet)
            .service(web::scope("/hello").configure(|cfg| {
                app::hello::make_config(cfg, &counter);
            }))
            .configure(app::config)
            .service(web::scope("/scopedapi").configure(app::scoped_config))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
