use tonic_build;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(&["api/messages.proto"], &["api"])?;
    // tonic_build::compile_protos("src/messages.proto")?;
    Ok(())
}
