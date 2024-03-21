mod echo;

use actix_web::web;

pub fn wsapis_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/ws")
                .configure(echo::echo_config)
        );
}