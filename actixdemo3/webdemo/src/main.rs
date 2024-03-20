use lib_demo::init_env_logger;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    init_env_logger();
    log::info!("start");

    Ok(())
}
