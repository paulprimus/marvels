use std::path::Path;

fn main() {
    // unsafe {
    //     std::env::set_var("PROTOC", "C:/javadev/tools/protoc/bin/protoc");
    // }
    let src = Path::new("./src/");

    let mut config = prost_build::Config::new();
    config.protoc_executable("/opt/tools/protoc/bin/protoc");
    config.btree_map(["."]);
    prost_build::compile_protos(
        &["/home/paul/projekte/rust/marvels/proto/authentication.proto"],
        &[src],
    )
    .expect("Failed to compile protos");
}
