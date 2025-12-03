use clap::{Parser, Subcommand, Args};


/// Doc comment
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Cli {
    /// Doc comment
    #[command(subcommand)]
    pub(crate) command: MarvelCommand,
}

#[derive(Subcommand, Debug, Clone)]
pub(crate) enum MarvelCommand {
    /// Start the server
    server,
    /// Start the client
    client
}
