use server::run_server;

#[tokio::main]
async fn main() {
    if let Err(e) = run_server().await {
        eprintln!("Server error: {e}");
        std::process::exit(1);
    }
}

