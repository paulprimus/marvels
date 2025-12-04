use clap::Parser;
use server;
use core::MarvelError;

mod cli;

fn main() -> Result<(), MarvelError> {

    let args = cli::Cli::parse();
    match args.command {


        cli::MarvelCommand::server => {
            server::server::run_server()
        },
        cli::MarvelCommand::client => {
            dbg!("Starting client...");
            client::client::authenticate("userID1", "password1")
        },

    }

}
