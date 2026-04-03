use client::run_authenticate;

#[tokio::main]
async fn main() {
    // TODO: Argumente per CLI einlesen (z.B. mit clap)
    if let Err(e) = run_authenticate("userID1", "password1").await {
        eprintln!("Client error: {e}");
        std::process::exit(1);
    }
}

