extern crate protoc_rust;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src",
        input: &["src/types.proto"],
        ..Default::default()
    }).expect("Expected to find types.proto at src/types.proto.");
}
