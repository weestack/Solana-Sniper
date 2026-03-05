mod oracle;

use solana_client::rpc_config::CommitmentConfig;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::task::JoinHandle;
use utils::dex::dex::MintTransaction;
use utils::env::env::Env;
use utils::ring_buffer::ring_buffer::RingBuffer;

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

    /* setup ring buffer */

    let ring_buffer = Arc::new(RingBuffer::new(1024));
    let mut workers: Vec<JoinHandle<usize>> = Vec::with_capacity(4);
    let processed_count = Arc::new(AtomicUsize::new(0));

    /* spawn sniper */
    for _ in 0..4 {
        let rb: Arc<RingBuffer<MintTransaction>> = Arc::clone(&ring_buffer);
        let pc = Arc::clone(&processed_count);
        let sniper = tokio::spawn(async move {
            loop {
                if let Some(task) = rb.dequeue() {
                    pc.fetch_add(1, Ordering::Relaxed);
                    println!("Sniped coin: {:?}", task);
                } else {
                    tokio::task::yield_now().await;
                }
            }
        });
        workers.push(sniper);
    }

    /* spawn listeners */
    println!(
        "Ring buffer created with capacity: {}",
        ring_buffer.buffer.capacity()
    );

    let producer_rb = Arc::clone(&ring_buffer);
    /* note we could also be starting 4 seperate threads here with pumpfun, orca, solarflare and raydium */
    tokio::spawn(async move {
        let subscribe_to_raydium = vec!["675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8".to_string()];

        /* Listen for all events regarding Raydium */
        let subscriber = oracle::websocket::SolanaSubscriber::new(
            env.websocket_endpoint,
            env.rpc_endpoint,
            CommitmentConfig::confirmed(),
            subscribe_to_raydium,
        )
            .await;

        subscriber.start_thread(producer_rb).await;
    });
    for handle in workers {
        handle.await.expect("Sniper thread failed");
    }
}
