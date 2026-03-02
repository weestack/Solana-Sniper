mod oracle;

use solana_client::rpc_config::CommitmentConfig;
use utils::env::env::Env;

#[tokio::main]
async fn main() {
    /* Load in ENV from .env, or suggest creating from .env.dist */
    let env = match Env::new() {
        Ok(env) => env,
        Err(e) => {
            println!("Error loading ENV: {}", e);
            std::process::exit(1);
        }
    };

    env.setup_logger();

    let subscribe_to_raydium = vec!["675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8".to_string()];

    /* Listen for all events regarding Raydium */
    let subscriber = oracle::websocket::SolanaSubscriber::new(
        env.websocket_endpoint,
        env.rpc_endpoint,
        CommitmentConfig::confirmed(),
        subscribe_to_raydium
    ).await;

    subscriber.start_thread().await;
}

