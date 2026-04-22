use server::run_server;
use rustls;
#[tokio::main]
async fn main() {
    // jsonwebtoken v10 benötigt einen explizit installierten CryptoProvider

    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("CryptoProvider konnte nicht installiert werden");

    if let Err(e) = run_server().await {
        eprintln!("Server error: {e}");
        std::process::exit(1);
    }
}

