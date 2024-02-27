use std::sync::{Mutex,Arc};
use actix_web::{
    get,
    post,
    web,
    rt::time,
    HttpResponse,
    Responder,
    Error,
};
use actix_session::{
    Session,
};
use std::time::Duration;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex 线程安全
}

#[get("")]
async fn hello(data: web::Data<AppStateWithCounter>, session: Session) -> Result<HttpResponse, Error> {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    if let Some(count) = session.get::<i32>("counter")? {
        session.insert("counter", count + 1)?;
    } else {
        session.insert("counter", 1)?;
    }

    Ok(
        HttpResponse::Ok()
            .body(
                format!(
                    "Request total number: {counter}  session number: {:?}",
                    session.get::<i32>("counter")?.unwrap()
                )
            )
    )
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    // std::thread::sleep(Duration::from_secs(5)); // 线程方式虽然不依赖特定的异步库，但是不好。
    time::sleep(Duration::from_secs(5)).await; // 等待要用 异步方式。
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// pub fn make_config() -> Arc<dyn for<'a> Fn(&'a mut web::ServiceConfig) + Send + 'static>  {
pub fn make_config() -> Arc<dyn Fn(& mut web::ServiceConfig) + Send + Sync + 'static>  {
// pub fn make_config() -> Arc<Box<dyn for<'a> Fn(&'a mut web::ServiceConfig)>>  {
    log::info!("on make_config");

    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    Arc::new(move | cfg: &mut web::ServiceConfig | {
        cfg.app_data(counter.clone())
        .service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello));
    })
}