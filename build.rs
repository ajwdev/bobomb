fn main() {
    protoc_rust_grpc::Codegen::new()
        .out_dir("src/nes/debugger/")
        .input("src/nes/debugger/debugger_server.proto")
        .rust_protobuf(true)
        .run()
        .expect("protoc-rust-grpc");
}
