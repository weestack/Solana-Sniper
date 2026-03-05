use std::str::FromStr;
use std::sync::Arc;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
};

use futures::StreamExt;
use log::{error, info};
use solana_client::rpc_config::CommitmentConfig;
use solana_sdk::signature::Signature;
use utils::dex::dex::{MintTransaction, MintedTokenTransaction};
use utils::raydium::mint::RaydiumMintedTransaction;
use utils::ring_buffer::ring_buffer::RingBuffer;

pub struct SolanaSubscriber {
    client: PubsubClient,
    config_level: CommitmentConfig,
    subscribe_to: Vec<String>,
    rpc_endpoint: String,
}


impl SolanaSubscriber {
    pub async fn new(ws_url: Arc<String>, rpc_endpoint: Arc<String>, config_level: CommitmentConfig, subscribe_to: Vec<String>) -> Self {
        let client = PubsubClient::new(ws_url.clone().as_str())
            .await
            .expect(format!("Failed to connect to {}", ws_url).as_str());

        SolanaSubscriber {
            client,
            config_level,
            subscribe_to,
            rpc_endpoint: rpc_endpoint.clone().to_string(),
        }
    }

    pub async fn start_thread(&self, ring_buffer: Arc<RingBuffer<MintTransaction>>) {
        info!("Starting Solana websocket subscriber");
        let subscribe_to = self.subscribe_to.clone();
        let config_level = self.config_level.clone();
        let rpc_endpoint = self.rpc_endpoint.clone();
        let (mut stream, _) = self.client.logs_subscribe(
            RpcTransactionLogsFilter::Mentions(
                subscribe_to
            ),
            RpcTransactionLogsConfig {
                commitment: Some(config_level),
            }
        ).await.expect("Failed to subscribe to Logs!");

        info!("Waiting for next event");
        while let Some(response) = stream.next().await {

            for log in &response.value.logs {
                /* skip all events that does not contain initialize2 eg token create */
                if !log.contains("initialize2") {
                    continue
                }
                {
                    let tx = Signature::from_str(response.value.signature.as_str()).unwrap();
                    info!("Received tx https://solscan.io/tx/{}", tx);
                    let transaction = RaydiumMintedTransaction::get_transaction(tx, rpc_endpoint.clone()).await;
                    
                    if transaction.is_err() {
                        error!("Failed to get transaction");
                    } else {
                        let initialize2_transaction = transaction.unwrap();
                        let state = ring_buffer.enqueue(
                            initialize2_transaction.clone()
                        );
                        
                        if state.is_err() {
                            error!("Failed to enqueue transaction");
                        } else {
                            info!("=====Enqueued transaction: {}=====\r\n{}", initialize2_transaction.get_mint(), initialize2_transaction);
                        }
                    }
                }
            }
        }

        info!("Stopping Solana websocket subscriber, perhaps an error with the subscriber");
    }
}