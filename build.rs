fn main() {
    protoc_rust_grpc::Codegen::new()
        .out_dir("src/proto")
        .input("src/proto/profanity.proto")
        .rust_protobuf(true)
        .run()
        .expect("protoc-rust-grpc");
}