use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::time::{Instant};

#[derive(Debug)]
pub struct MocketSwapTask {
    pub mint: String,
    pub dex: String,
}

struct Slot {
    sequence: AtomicUsize,
    data: std::cell::UnsafeCell<Option<MocketSwapTask>>,
}

unsafe impl Send for Slot {}
unsafe impl Sync for Slot {}

pub struct RingBuffer {
    buffer: Vec<Slot>,
    mask: usize,
    enqueue_pos: AtomicUsize,
    dequeue_pos: AtomicUsize,
}

impl RingBuffer {
    pub fn new(size: usize) -> Self {
        assert!(size.is_power_of_two(), "Size must be a power of two");

        let mut buffer = Vec::with_capacity(size);
        for i in 0..size {
            buffer.push(Slot {
                sequence: AtomicUsize::new(i),
                data: std::cell::UnsafeCell::new(None),
            });
        }

        Self {
            buffer,
            mask: size - 1,
            enqueue_pos: AtomicUsize::new(0),
            dequeue_pos: AtomicUsize::new(0)
        }
    }

    pub fn enqueue(&self, data: MocketSwapTask) -> Result<(), MocketSwapTask> {
        let mut pos = self.enqueue_pos.load(Ordering::Relaxed);
        loop {
            let slot = &self.buffer[pos & self.mask];
            let seq = slot.sequence.load(Ordering::Acquire);
            let diff = seq as i64 - pos as i64;

            if diff == 0 {
                if self.enqueue_pos.compare_exchange_weak(pos, pos + 1, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
                    unsafe {
                        *slot.data.get() = Some(data);
                        slot.sequence.store(pos + 1, Ordering::Release);
                        return Ok(());
                    }
                }
            } else if diff < 0 {
                /* Queue full, tell the caller and let them handle it */
                return Err(data);
            } else {
                pos = self.enqueue_pos.load(Ordering::Relaxed);
            }
        }
    }

    pub fn dequeue(&self) -> Option<MocketSwapTask> {
        let mut pos = self.dequeue_pos.load(Ordering::Relaxed);
        loop {
            let slot = &self.buffer[pos & self.mask];
            let seq = slot.sequence.load(Ordering::Acquire);
            let diff = seq as i64 - (pos + 1) as i64;

            if diff == 0 {
                if self.dequeue_pos.compare_exchange_weak(pos, pos + 1, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
                    let data = unsafe { (*slot.data.get()).take() };
                    slot.sequence.store(pos + self.buffer.len(), Ordering::Release);
                    return data;
                }
            } else if diff < 0 {
                /* Queue empty, tell the caller and let them handle it */
                return None;
            } else {
                pos = self.dequeue_pos.load(Ordering::Relaxed);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio::task::JoinHandle;

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