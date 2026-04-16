mod client;

#[tokio::main]
async fn main() {
    // TODO: Argumente per CLI einlesen (z.B. mit clap)
    let e = client::authenticate("userID1", "password1").await;
    match e {
        Ok(v) => println!("{}", v),
        Err(e) => println!("Fehler: {}", e),
    }
}
