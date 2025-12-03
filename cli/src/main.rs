use clap::Parser;
use server;
use server::server::MarvelError;

mod cli;

fn main() -> Result<(), MarvelError> {

    let args = cli::Cli::parse();
    match args.command {


        cli::MarvelCommand::server => {
            server::server::run_server()
        },
        cli::MarvelCommand::client => {
            println!("Starting client...");
            Ok(())
        },

    }

}
