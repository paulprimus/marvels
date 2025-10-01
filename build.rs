use std::path::Path;

fn main() {

    let src = Path::new("./src");

    let mut config = prost_build::Config::new();
    config.btree_map(["."]);
    prost_build::compile_protos(&["proto/authentication.proto"], &[src]).expect("Failed to compile protos");

}