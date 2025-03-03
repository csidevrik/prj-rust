use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = PathBuf::from("proto/agent.proto");
    let proto_dir = PathBuf::from("proto");

    println!("cargo:rerun-if-changed=proto/agent.proto");
    
    if !proto_file.exists() {
        println!("cargo:warning=Proto file not found at {:?}", proto_file);
        return Err("Proto file not found".into());
    }

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(&[proto_file], &[proto_dir])?;

    Ok(())
} 