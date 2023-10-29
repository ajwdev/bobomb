fn main() -> Result<(), Box<dyn std::error::Error>> {
    lalrpop::process_root().unwrap();

    tonic_build::configure()
        .out_dir("src/grpc")
        .build_server(true)
        .build_client(true)
        .compile(&["proto/api.proto"], &["proto"])?;
    Ok(())
}
