extern crate protobuf_codegen_pure;

fn main() {
    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src/proto",
        input: &[
            "protobuf/abci.proto",
            "protobuf/github.com/tendermint/tendermint/libs/kv/types.proto",
            "protobuf/github.com/tendermint/tendermint/crypto/merkle/merkle.proto",
        ],
        includes: &["protobuf"],
        customize: protobuf_codegen_pure::Customize {
            ..Default::default()
        },
    })
    .expect("protoc");
}
