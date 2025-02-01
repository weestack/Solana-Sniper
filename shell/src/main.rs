use clap::{Parser, Subcommand};
use utils::env::env::Env;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "Solana Commands")]
#[command(about = "CLI solana commands", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Clone, Debug, Subcommand)]
enum Commands {
    ListAccount,
    //WrapSol(wrap_sol::WrapArgs),
    //CloseAccount(close_account::AccountArgs)
}


fn main() {
    let env = Env::new().unwrap();
    let cli_command = Cli::parse();

    match cli_command.command {
        Commands::ListAccount => {
           println!("List account");
        }
    }
}
