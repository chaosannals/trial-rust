use chrono::Local;
use env_logger::Builder;

fn main() {
    Builder::new()
        .init();
    let now = Local::now().naive_local();
    log::info!("now: {:?}", now);
}
