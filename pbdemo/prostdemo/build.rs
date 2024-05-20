fn main() {
    let mut protos_dir = std::env::current_dir().expect("获取当前目录失败。");
    protos_dir.pop();
    protos_dir.push("protos");
    let echo_proto_path = protos_dir.join("echo.proto");

    prost_build::Config::new()
        .compile_protos(&[echo_proto_path], &[protos_dir])
        .unwrap();
}