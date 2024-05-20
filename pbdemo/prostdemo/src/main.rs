mod prost_protos {
    include!(concat!(env!("OUT_DIR"), "/trial.rust.pbdemo.echo.rs"));
}


fn main() {
    let mut req = prost_protos::EchoRequest::default();
    req.message = "213432".to_owned();
    println!("the: {:?}", req);
    // println!("the: {:}", req); // 没有定义 display
}
