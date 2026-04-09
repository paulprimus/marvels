fn main() {
    println!("cargo:rerun-if-changed=src/security.proto");

    let src_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("src");

    let mut config = prost_build::Config::new();

    // Nur nötig wenn PROTOC auf ein Verzeichnis zeigt – prost-build liest PROTOC als Executable selbst
    if let Ok(protoc) = std::env::var("PROTOC") {
        let path = std::path::PathBuf::from(protoc);
        if path.is_dir() {
            let exe = if cfg!(windows) { "protoc.exe" } else { "protoc" };
            let bin = if path.ends_with("bin") { path } else { path.join("bin") };
            config.protoc_executable(bin.join(exe));
        }
    }

    config
        .compile_protos(
            &[src_dir.join("security.proto").to_string_lossy().as_ref()],
            &[src_dir.to_string_lossy().as_ref()],
        )
        .expect("Generierung von authentication protos fehlgeschlagen");
}
