use chrono::Local;

fn main() {
    let now = Local::now().naive_local();
    println!("now: {:?}", now);
}
