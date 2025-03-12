mod commands;
use commands::wrap_sol::WrapArgs;
use clap::{Parser, Subcommand};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use utils::env::env::Env;
use crate::commands::wrap_sol;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "Solana Commands")]
#[command(about = "CLI solana commands", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Clone, Debug, Subcommand)]
enum Commands {
    WrapSol(WrapArgs),
}

#[tokio::main]
async fn main() {
    let env = Env::new().unwrap();
    let cli_command = Cli::parse();

    match cli_command.command {
        Commands::WrapSol(wrap_args) => {
            let amount = (wrap_args.amount as f64 * LAMPORTS_PER_SOL as f64) as u64;

            wrap_sol::wrap_sol_fn(
                amount,
                &env.private_key,
                env.rpc_endpoint.to_string(),
            ).await
        }
    }
}
