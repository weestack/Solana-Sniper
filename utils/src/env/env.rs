use std::{env, fmt, fs, io};
use std::path::Path;
use std::sync::Arc;
use solana_program::native_token::lamports_to_sol;
use crate::env::errors::EnvErrors;

pub struct Env {
    websocket_endpoint: Arc<String>,
    rpc_endpoint: Arc<String>,
    private_key: Arc<String>,

    swap_amount: Arc<u64>,
    swap_priority_fee: Arc<u64>,
}

impl Env {
    pub fn new() -> Result<Self, EnvErrors> {
        // Test if .env exists or give the option to create one from .env.dist
        let path = Path::new(".env");
        if ! path.exists() {
            println!("Could not find .env, would you like to use .env.dist as a template instead? (y/n)");
            create_env().expect("Failed creating .env file");
        }
        dotenv::from_path(".env").ok();

        let websocket_endpoint = Arc::new(env::var("WEBSOCKET_ENDPOINT")?);
        let rpc_endpoint = Arc::new(env::var("RPC_ENDPOINT")?);
        let private_key = Arc::new(env::var("PRIVATE_KEYPAIR")?);

        let swap_amount = Arc::new(env::var("SWAP_AMOUNT")?.parse::<u64>().unwrap());
        let swap_priority_fee = Arc::new(env::var("SWAP_PRIORITY_FEE")?.parse::<u64>().unwrap());

        Ok(
            Self {
                websocket_endpoint,
                rpc_endpoint,
                private_key,
                swap_amount,
                swap_priority_fee,
            }
        )
    }
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "===================== ENVS Loaded =====================\r\n\
        websocket_endpoint: {:?}\r\n\
        rpc endpoint:       {:?}\r\n\
        private wallet:     {:?}\r\n\
        swap amount:        {:?} sol\r\n\
        swap_priority_fee:  {:?} sol\r\n\
        total per trade:    {:?} sol\r\n\
        =====================================================",
                 self.websocket_endpoint,
                 self.rpc_endpoint,
                 self.private_key,
                 lamports_to_sol(*self.swap_amount),
                 lamports_to_sol(*self.swap_priority_fee),
                 lamports_to_sol(*self.swap_amount.clone() + *self.swap_priority_fee.clone())
        )
    }
}

fn create_env() -> std::io::Result<()> {
    let source = ".env.dist";
    let destination = ".env";

    println!("Do you want to copy '{}' to '{}' (y/n)?", source, destination);

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Trim input and check the response
    if input.trim().eq_ignore_ascii_case("y") {
        // Perform the copy
        fs::copy(source, destination)?;
        println!("File successfully copied to '{}'.", destination);
    } else {
        println!("Operation canceled.");
    }

    Ok(())
}
