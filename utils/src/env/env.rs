use std::{env, fmt, fs, io};
use std::path::Path;
use std::sync::Arc;
use solana_program::native_token::lamports_to_sol;
use crate::env::errors::EnvErrors;
use env_logger::{Builder, Env as EnvBuilder};
use log::{LevelFilter};

pub struct Env {
    pub loglevel: Arc<LevelFilter>,
    pub websocket_endpoint: Arc<String>,
    pub rpc_endpoint: Arc<String>,
    pub private_key: Arc<String>,
    pub swap_amount: Arc<u64>,
    pub swap_priority_fee: Arc<u64>,
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

        let loglevel = Arc::new(parse_log_level(env::var("LOG_LEVEL")?));

        let websocket_endpoint = Arc::new(env::var("WEBSOCKET_ENDPOINT")?);
        let rpc_endpoint = Arc::new(env::var("RPC_ENDPOINT")?);

        let private_key = Arc::new(env::var("PRIVATE_KEYPAIR")?);
        let swap_amount = Arc::new(env::var("SWAP_AMOUNT")?.parse::<u64>().unwrap());
        let swap_priority_fee = Arc::new(env::var("SWAP_PRIORITY_FEE")?.parse::<u64>().unwrap());

        Ok(
            Self {
                loglevel,
                websocket_endpoint,
                rpc_endpoint,
                private_key,
                swap_amount,
                swap_priority_fee,
            }
        )
    }

    pub fn setup_logger(&self) {
        let env = EnvBuilder::default();

        Builder::from_env(env)
            .filter_level(*self.loglevel)
            .format_level(false)
            .format_timestamp_millis()
            .init();
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


fn parse_log_level(log_level: String) -> LevelFilter {
    match log_level.to_lowercase().as_str() {
        "off" => LevelFilter::Off,
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info // defaults to info, if wrong loglevel given
    }
}