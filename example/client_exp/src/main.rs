use clap::{Parser, Subcommand};
use marvels_client::MarvelsClient;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Authenticate {
        #[arg(short, long)]
        client_id: String,
        #[arg(short, long)]
        secret: String,
    },
    Authorize {
        #[arg(short, long)]
        auth_code: String,
        #[arg(short, long)]
        verifier_code: String,
        #[arg(short, long)]
        client_id: String,
        #[arg(short, long)]
        scope: String,
    },
    Protected {
        #[arg(short, long)]
        access_token: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = MarvelsClient::new("http://localhost:3000");

    match cli.command {
        Commands::Authenticate { client_id, secret } => {
            match client.authenticate(&client_id, &secret).await {
                Ok(auth) => println!(
                    "Auth code: {}, Verifier: {}",
                    auth.auth_code, auth.code_verifier
                ),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Authorize {
            auth_code,
            verifier_code,
            client_id,
            scope,
        } => {
            match client
                .authorize(&auth_code, &verifier_code, &client_id, &scope)
                .await
            {
                Ok(access_token) => println!("Access_token: {}", access_token),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Protected { access_token } => match client.call_protected(&access_token).await {
            Ok(_) => println!("Protected call successful"),
            Err(e) => eprintln!("Error: {}", e),
        },
    }
}
