#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use solana_program::pubkey;
    use solana_sdk::pubkey::Pubkey;
    use tokio::time::Instant;
    use tokio::task::JoinHandle;
    use crate::dex::dex::{MintTransaction, MintedTokenTransaction};
    use crate::orca::mint::OrcaMintedTransaction;
    use crate::pumpfun::mint::PUMPFUNMintedTransaction;
    use crate::raydium::mint::RaydiumMintedTransaction;
    use crate::ring_buffer::ring_buffer::RingBuffer;
    use crate::solarflare::mint::SolarFlareMintedTransaction;

    #[tokio::test]
    async fn test_high_throughput_queue() {
        let num_messages = 10_000;
        let buffer_size = 1024;

        // 1. Initialize RingBuffer with the Enum type
        let ring_buffer = Arc::new(RingBuffer::<MintTransaction>::new(buffer_size));
        let processed_count = Arc::new(AtomicUsize::new(0));
        let start_time = Instant::now();

        let mut workers: Vec<JoinHandle<usize>> = Vec::with_capacity(4);

        // Consumers to perform the snipe/swap
        for _ in 0..4 {
            let rb = Arc::clone(&ring_buffer);
            let pc = Arc::clone(&processed_count);

            let handle = tokio::spawn(async move {
                let mut local_count = 0;
                loop {
                    if let Some(transaction) = rb.dequeue() {
                        local_count += 1;

                        // Just an example of calling a trait method on the dequeued enum
                        let _dex = transaction.get_dex();

                        pc.fetch_add(1, Ordering::Relaxed);
                    } else {
                        if pc.load(Ordering::Relaxed) >= num_messages {
                            break;
                        }
                        tokio::task::yield_now().await;
                    }
                }
                local_count
            });
            workers.push(handle);
        }

        // Producers to generate the transactions on newly minted tokens
        let producer_rb = Arc::clone(&ring_buffer);
        tokio::spawn(async move {
            for i in 0..num_messages {
                // Cycle through the 4 different transaction types
                let mut tx = match i % 3 {
                    // Use the new mock() method for Raydium
                    //1 => MintTransaction::RAYDIUM(RaydiumMintedTransaction::mock()),

                    // Mocks for others using unique Pubkeys
                    0 => MintTransaction::ORCA(OrcaMintedTransaction {
                        mint: Pubkey::new_unique()
                    }),
                    1 => MintTransaction::PUMPFUN(PUMPFUNMintedTransaction {
                        mint: Pubkey::new_unique()
                    }),
                    _ => MintTransaction::SOLARFLARE(SolarFlareMintedTransaction {
                        mint: Pubkey::new_unique()
                    }),
                };

                loop {
                    // Try to enqueue; if full, the loop yields and tries again
                    match producer_rb.enqueue(tx) {
                        Ok(_) => break,
                        Err(returned_tx) => {
                            tx = returned_tx;
                            tokio::task::yield_now().await;
                        }
                    }
                }
            }
        });

        // --- Wait for completion ---
        let mut total_from_workers = 0;
        for handle in workers {
            total_from_workers += handle.await.expect("Worker thread failed");
        }

        let duration = start_time.elapsed();
        println!("\n--- Multi-DEX Queue Results ---");
        println!("Total messages processed: {}", total_from_workers);
        println!("Time taken: {:?}", duration);
        println!("Throughput: {:.2} msg/sec", num_messages as f64 / duration.as_secs_f64());

        assert_eq!(total_from_workers, num_messages);
    }
}