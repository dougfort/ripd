fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("build: ipd.proto");
    tonic_build::compile_protos("proto/ipd.proto")?;
    Ok(())
}
