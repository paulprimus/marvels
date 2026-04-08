
fn main() {
    println!("cargo:rerun-if-changed=security.proto");

    // Manifest-Verzeichnis (wo Cargo.toml der proto crate ist)
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let proto_path = std::path::PathBuf::from(&manifest_dir).join("security.proto");
    let include_path = std::path::PathBuf::from(&manifest_dir).join("src");

    // PROTOC-Pfad auflösen, unterstützt Windows und Linux
    if let Ok(protoc_path) = std::env::var("PROTOC") {
        let path = std::path::PathBuf::from(&protoc_path);

        let protoc_exe = if path.is_dir() {
            // Wenn PROTOC ein Verzeichnis ist, suchen wir das protoc-Executable
            let exe_name = if cfg!(windows) { "protoc.exe" } else { "protoc" };

            // Prüfe ob bereits ein bin-Verzeichnis in diesem Pfad ist
            let candidate = if path.ends_with("bin") {
                path.join(exe_name)
            } else {
                path.join("bin").join(exe_name)
            };

            println!("cargo:warning=PROTOC-Pfad: {}", candidate.display());
            candidate
        } else {
            path
        };

        let mut config = prost_build::Config::new();
        config.protoc_executable(protoc_exe);

        config
            .compile_protos(&[proto_path.to_string_lossy().as_ref()], &[include_path.to_string_lossy().as_ref()])
            .expect("Generierung von authentication protos fehlgeschlagen");
    } else {
        // Fallback: prost-build findet protoc automatisch
        let mut config = prost_build::Config::new();
        config
            .compile_protos(&[proto_path.to_string_lossy().as_ref()], &[include_path.to_string_lossy().as_ref()])
            .expect("Generierung von authentication protos fehlgeschlagen");
    }
}



