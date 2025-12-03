
fn main()  {
    dbg!("Build-RS gestartet");
    prost_build::compile_protos(&["src/security.proto"], &["src"])
        .expect("Generierung von authentication protos fehlgeschlagen");

}
