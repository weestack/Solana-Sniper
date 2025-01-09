use std::sync::Arc;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
};
use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};
use futures::StreamExt;
use log::info;

pub struct SolanaSubscriber {
    client: PubsubClient,
    config_level: CommitmentConfig,
    subscribe_to: Vec<String>
}


impl SolanaSubscriber {
    pub async fn new(ws_url: Arc<String>, config_level: CommitmentConfig, subscribe_to: Vec<String>) -> Self {
        let client = PubsubClient::new(ws_url.clone().as_str())
            .await
            .expect(format!("Failed to connect to {}", ws_url).as_str());

        SolanaSubscriber {
            client,
            config_level,
            subscribe_to
        }
    }

    pub async fn start_thread(&self) {
        info!("Starting Solana websocket subscriber");
        let subscribe_to = self.subscribe_to.clone();
        let config_level = self.config_level.clone();
        let (mut stream, _) = self.client.logs_subscribe(
            RpcTransactionLogsFilter::Mentions(
                subscribe_to
            ),
            RpcTransactionLogsConfig {
                commitment: Some(config_level),
            }
        ).await.expect("Failed to subscribe to Logs!");

        loop {
            info!("Waiting for next event");
            while let Some(response) = stream.next().await {

                for log in &response.value.logs {
                    /* skip all events that does not contain initialize2 eg token create */
                    if !log.contains("initialize2") {
                        continue
                    }
                    info!("Received tx https://solscan.io/tx/{}", response.value.signature);
                }
            }
        }
    }
}