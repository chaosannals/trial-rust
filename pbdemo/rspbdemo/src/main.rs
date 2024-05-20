mod rust_protobuf_protos {
    include!(concat!(env!("OUT_DIR"), "/rust_protobuf_protos/mod.rs"));
}

fn main() {
    let mut req = rust_protobuf_protos::echo::EchoRequest::new();
    req.message = "13456".to_owned();
    println!("the: {:?}", req);
    println!("the: {:}", req);
}
