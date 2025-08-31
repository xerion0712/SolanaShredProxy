fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build protobuf files
    tonic_build::compile_protos("proto/solana_shred.proto")?;
    
    // Re-run if proto files change
    println!("cargo:rerun-if-changed=proto/");
    
    Ok(())
}
