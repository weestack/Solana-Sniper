use std::sync::atomic::{AtomicUsize, Ordering};
use crate::dex::dex::MintedTokenTransaction;

pub struct Slot<T: MintedTokenTransaction> {
    sequence: AtomicUsize,
    data: std::cell::UnsafeCell<Option<T>>,
}

unsafe impl<T: MintedTokenTransaction + Send> Send for Slot<T> {}
unsafe impl<T: MintedTokenTransaction + Sync> Sync for Slot<T> {}

pub struct RingBuffer<T: MintedTokenTransaction> {
    pub buffer: Vec<Slot<T>>,
    mask: usize,
    enqueue_pos: AtomicUsize,
    dequeue_pos: AtomicUsize,
}

impl<T: MintedTokenTransaction> RingBuffer<T> {
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

    pub fn enqueue(&self, data: T) -> Result<(), T> {
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

    pub fn dequeue(&self) -> Option<T> {
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

