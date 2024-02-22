use actix_web::{web, App, HttpServer};

mod app;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(&move || {
        let hello_config = app::hello::make_config();
        App::new()
            .configure(app::config)
            .service(web::scope("/hello").configure(hello_config))
            .service(web::scope("/scopedapi").configure(app::scoped_config))
    })
    .bind(("0.0.0.0", 44444))?
    .run()
    .await
}