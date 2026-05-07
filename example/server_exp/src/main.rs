use clap::{Parser, Subcommand};
use marvels_auth::AuthRouterBuilder;
use tokio;
use tracing::info;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Start => {
            let router = AuthRouterBuilder::new()
                .jwt_secret(b"secret")
                .token_expiry(3600)
                .build();

            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
            info!("Listening on http://0.0.0.0:3000");
            axum::serve(listener, router).await.unwrap();
        }
    }
}
