
fn main()  {
    dbg!("Build-RS gestartet");
    prost_build::compile_protos(&["./authentication.proto"], &["./"])
        .expect("Generierung von authentication protos fehlgeschlagen");

}
