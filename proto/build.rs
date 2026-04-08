fn main()  {
    eprintln!("Build-RS gestartet");
    prost_build::compile_protos(&["security.proto"], &["."])
        .expect("Generierung von authentication protos fehlgeschlagen");

}
