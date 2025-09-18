use std::io::Result;

fn main() -> Result<()> {
    // Tell Cargo to rerun this build script if the proto file changes
    println!("cargo:rerun-if-changed=proto/chunk_metadata.proto");

    // Use vendored protoc
    unsafe {
        std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path().unwrap());
    }
    
    prost_build::compile_protos(&["proto/chunk_metadata.proto"], &["proto/"])?;
    Ok(())
}