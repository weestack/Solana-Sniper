#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use tokio::time::{Instant};
    use tokio::task::JoinHandle;
    use crate::ring_buffer::ring_buffer::{MocketSwapTask, RingBuffer};

    #[tokio::test]
    async fn test_high_throughput_queue() {
        let num_messages = 10_000;
        let buffer_size = 1024;
        let ring_buffer = Arc::new(RingBuffer::new(buffer_size));
        let processed_count = Arc::new(AtomicUsize::new(0));
        let start_time = Instant::now();

        let mut workers: Vec<JoinHandle<usize>> = Vec::with_capacity(4);

        for _ in 0..4 {
            let rb = Arc::clone(&ring_buffer);
            let pc = Arc::clone(&processed_count);

            let handle = tokio::spawn(async move {
                let mut local_count = 0;
                loop {
                    if let Some(_task) = rb.dequeue() {
                        local_count += 1;
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

        // 2. Producer: Bursting 10k messages
        let producer_rb = Arc::clone(&ring_buffer);
        tokio::spawn(async move {
            for i in 0..num_messages {
                let mut task = MocketSwapTask {
                    mint: format!("Mint_{}", i),
                    dex: "Raydium".to_string(),
                };

                loop {
                    match producer_rb.enqueue(task) {
                        Ok(_) => break,
                        Err(returned_task) => {
                            task = returned_task;
                            tokio::task::yield_now().await;
                        }
                    }
                }
            }
        });


        let mut total_from_workers = 0;
        for handle in workers {
            total_from_workers += handle.await.expect("Worker thread failed");
        }

        let duration = start_time.elapsed();
        println!("\n--- Sniper Queue Results ---");
        println!("Total messages processed: {}", total_from_workers);
        println!("Time taken: {:?}", duration);
        println!("Throughput: {:.2} msg/sec", num_messages as f64 / duration.as_secs_f64());

        assert_eq!(total_from_workers, num_messages);
    }
}