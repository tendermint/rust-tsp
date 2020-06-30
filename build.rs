extern crate protobuf_codegen_pure;

fn main() {
    protobuf_codegen_pure::Codegen::new()
        .out_dir("src/messages")
        .inputs(&[
            "protobuf/abci.proto",
            "protobuf/libs/kv/types.proto",
            "protobuf/crypto/merkle/merkle.proto",
        ])
        .include("protobuf")
        .run()
        .expect("protoc");
}
