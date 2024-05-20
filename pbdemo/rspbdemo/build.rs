fn main() {
    let mut protos_dir = std::env::current_dir().expect("获取当前目录失败。");
    protos_dir.pop();
    protos_dir.push("protos");
    let echo_proto_path = protos_dir.join("echo.proto");

    protobuf_codegen::Codegen::new()
        .include(protos_dir)
        .inputs([echo_proto_path])
        .cargo_out_dir("rust_protobuf_protos")
        .run_from_script();
}