
fn main() {
    dbg!("Build-RS gestartet");

    // Falls PROTOC auf ein Verzeichnis zeigt, automatisch auf bin/protoc.exe korrigieren
    let protoc_exe = std::env::var("PROTOC").ok().map(|p| {
        let path = std::path::PathBuf::from(&p);
        if path.is_dir() {
            let exe = path.join("bin").join("protoc.exe");
            println!("cargo:warning=PROTOC war ein Verzeichnis – korrigiert zu: {}", exe.display());
            exe
        } else {
            path
        }
    });

    let mut config = prost_build::Config::new();
    if let Some(exe) = protoc_exe {
        config.protoc_executable(exe);
    }

    config
        .compile_protos(&["src/security.proto"], &["src"])
        .expect("Generierung von authentication protos fehlgeschlagen");
}



