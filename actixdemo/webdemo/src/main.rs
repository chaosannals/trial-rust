use actix_web::{
    rt,
    dev,
    dev::Service as _,
    web,
    http::{
        KeepAlive,
        StatusCode,
        header,
        // header::{ContentEncoding},
    },
    App,
    HttpServer,
    HttpResponse,
    cookie::Key,
    // middleware,
    middleware::{
        ErrorHandlerResponse,
        ErrorHandlers,
        Logger,
        DefaultHeaders,
    },
    Result,
};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_files as fs;
// use actix_redis::RedisSessionBackend;
use futures_util::future::FutureExt;
// use std::time::Duration;
use std::{
    env,
    path::Path,
    time::Duration,
};

use sea_orm::{
    Database,
    DatabaseConnection,
};

mod app;
mod entities;
mod jobs;
mod service;
// mod tasks;

fn add_error_header<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("Error"), // content-type: error
    );

    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // 这个是运行目录。
    //log::info!("current workspace dir: {:?}", env::current_dir().unwrap());
    let exe_path_str = env::current_exe().unwrap();
    let exe_path = Path::new(&exe_path_str);
    let exe_dir = exe_path.parent().unwrap();
    log::info!("current exe dir: {:?}", exe_dir);
    let res_dir = exe_dir.join("res");
    log::info!("res dir: {:?}", res_dir);

    let env_path = exe_dir.join(".env");
    log::info!(".env path: {:?}", env_path);
    dotenvy::from_path(env_path).ok();

    // 这种返回 高阶函数的做法 Windows 下正常
    // 但是会导致 Windows 交叉编译 Linux 失败，可能是编译器不支持这么返回值。
    let hello_config = app::hello::make_config();
    let port = 44444;
    log::info!("starting HTTP server at http://localhost:{:?}", port);

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    log::info!("DB URL: {:?}", db_url);

    let conn = Database::connect(&db_url).await.unwrap();
    // Migrator::up(&conn, None).await.unwrap(); // 指定迁移。

    // 4 年没维护的库版本对不上 actix 了。
    // let queue = TaskQueue::<PlusFive>::from_registry();
    // let worker = TaskWorker::<PlusFive, PlusFiveResult>::new();

    // 直接通过协程启动 backend 事务。
    let (rte_jobs, rte_handle) = jobs::rt_ephemeral_jobs::start_queue();

    // app 状态
    let state = app::AppState { conn };

    let _ = HttpServer::new(move || {
        // 初始的时候会生成多个线程(默认好像是CPU线程数)相互独立，所以这里面的变量也是多份的且在内存常驻。
        let hello_config_arc = hello_config.clone(); // 异步引用，即使这里 clone 多分也只引用了外部的 hello_config
        
        log::info!("on new.");
        App::new()
            .app_data(web::Data::new(rte_jobs.clone()))
            .app_data(web::Data::new(state.clone()))
            .wrap(Logger::default())
            .wrap(DefaultHeaders::new().add(("X-Version", "0.2")))
            .wrap(service::hi::SayHi::default()) // 类定义的 wrap 全局加入起效
            // wrap_fn 直接挂闭包函数
            .wrap_fn(|req, srv| {
                log::info!("[fn] Hi from start. You requested: {}", req.path());
                srv.call(req).map(|res| {
                    log::info!("[fn] Hi from response");
                    res
                })
            })
            .wrap(
                // create cookie based session middleware
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build()
            )
            .wrap(
                ErrorHandlers::new()
                    .handler(StatusCode::INTERNAL_SERVER_ERROR, add_error_header),
            )
            .service(
                web::scope("/users")
                .configure(app::users::do_config),
            )
            .service(
                fs::Files::new("/res", res_dir.clone())
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .service(web::resource("/error_internal").route(web::get().to(HttpResponse::InternalServerError)))
            // .wrap(middleware::Compress::default())
            .configure(app::config)
            // configure 不支持 arc ，类型又解不出来，只能直接多套一层闭包（只是为了让类型对上。。）了。再调一次。
            .service(
                web::scope("/hello")
                .configure(|cfg| {hello_config_arc(cfg)})
            )
            .service(web::scope("/scopedapi").configure(app::scoped_config))
            .service(
                web::scope("/job")
                .configure(app::jobs::do_config)
            )
    })
    .workers(4) // 指定 workers 数量，默认是 CPU 线程数。
    // 指定 keep_alive 时间
    // .keep_alive(Duration::from_secs(75))
    .keep_alive(KeepAlive::Os)
    .bind(("0.0.0.0", port))?
    .run()
    .await;

    rte_handle.abort(); // CancellationToken 在 actix 下被合并到 handle 里了。
    let _ = rte_handle.await;

    Ok(())
}