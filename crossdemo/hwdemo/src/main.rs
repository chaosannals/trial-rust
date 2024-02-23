fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    println!("Hello, world!");
    log::info!("log start");
}
