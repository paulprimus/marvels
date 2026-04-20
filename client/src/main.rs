mod client;

use clap::{Parser, Subcommand};

/// Marvels OAuth 2.1 CLI-Client
#[derive(Parser)]
#[command(name = "client", version, about = "Marvels OAuth 2.1 CLI-Client")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Authentifizierung: client_id + client_secret → Authorization Code
    Authenticate {
        /// Client-ID
        #[arg(short = 'c', long)]
        client_id: String,

        /// Client-Secret
        #[arg(short = 's', long)]
        client_secret: String,
    },
    /// Autorisierung: Authorization Code → Access Token
    Authorize {
        /// Authorization Code (aus dem authenticate-Schritt)
        #[arg(short = 'q', long)]
        code: String,

        /// Code-Verifier (aus dem authenticate-Schritt)
        #[arg(short = 'v', long)]
        code_verifier: String,

        /// Client-ID
        #[arg(short = 'i', long)]
        client_id: String,

        /// Angefragte Scopes (leerzeichen-getrennt)
        #[arg(short = 's', long, default_value = "read")]
        scope: String,
    },
    /// Geschützte Ressource abrufen
    Protected {
        /// Bearer Access Token
        #[arg(short, long)]
        token: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Authenticate { client_id, client_secret } => {
            match client::authenticate(&client_id, &client_secret).await {
                Ok(result) => {
                    println!("Erfolgreich authentifiziert!");
                    println!("auth_code | code_verifier: {result}");
                    println!();
                    println!("Tipp: Nächster Schritt → authorize --code <auth_code> --code-verifier <verifier> --client-id {client_id}");
                }
                Err(e) => eprintln!("Fehler bei der Authentifizierung: {e}"),
            }
        }
        Commands::Authorize { code, code_verifier, client_id, scope } => {
            match client::authorize(&code, &code_verifier, &client_id, &scope).await {
                Ok(token) => {
                    println!("Access Token erhalten:");
                    println!("{token}");
                    println!();
                    println!("Tipp: Nächster Schritt → protected --token <access_token>");
                }
                Err(e) => eprintln!("Fehler bei der Autorisierung: {e}"),
            }
        }
        Commands::Protected { token } => {
            match client::call_protected(&token).await {
                Ok(body) => println!("{body}"),
                Err(e) => eprintln!("Fehler beim Zugriff auf geschützte Ressource: {e}"),
            }
        }
    }
}
