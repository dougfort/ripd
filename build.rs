// build.rs
// build script to compile protocol buffers definition

extern crate protoc_rust_grpc;

fn main() {
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src",
        includes: &[],
        input: &["ipd.proto"],
        rust_protobuf: true,
    }).expect("Failed to generate Rust src");
}