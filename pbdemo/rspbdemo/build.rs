fn main() {
    let mut protos_dir = std::env::current_dir().expect("获取当前目录失败。");
    protos_dir.pop();
    protos_dir.push("protos");
    let echo_proto_path = protos_dir.join("echo.proto");

    protobuf_codegen::Codegen::new()
        // .protoc() // 可以使用 protoc
        .pure() // 用纯 rust 实现替代 protoc
        .include(protos_dir)
        .inputs([echo_proto_path])
        .cargo_out_dir("rust_protobuf_protos")
        .run_from_script();
}
