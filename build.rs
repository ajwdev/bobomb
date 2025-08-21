fn main() -> Result<(), Box<dyn std::error::Error>> {
    lalrpop::Configuration::new()
        .set_in_dir("src")
        .process_dir("src")?;

    tonic_build::configure()
        .out_dir("src/grpc")
        .build_server(true)
        .build_client(true)
        .compile(&["proto/api.proto"], &["proto"])?;
    Ok(())
}
