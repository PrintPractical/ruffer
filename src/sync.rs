//! A Sync implementation of the RingBuffer.
//! This is just a helper struct which has a similar API and wraps the RingBuffer in a Mutex.
//! Utilize an std::sync::Arc to make a RingBuffer which is Send + Sync.

use crate::{RingBuffer, DEFAULT_CAPACITY};
use std::{
    io::{Read as _, Write as _},
    sync::Mutex,
};

pub struct SyncRingBuffer {
    buffer: Mutex<RingBuffer>,
}

// Static Impls
impl SyncRingBuffer {
    pub fn new() -> Self {
        SyncRingBuffer::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(size: usize) -> Self {
        SyncRingBuffer {
            buffer: Mutex::new(RingBuffer::with_capacity(size)),
        }
    }
}

// Member Impls
impl SyncRingBuffer {
    pub fn capacity(&self) -> usize {
        let buffer = self.buffer.lock().unwrap();
        buffer.capacity()
    }

    pub fn empty(&self) -> bool {
        let buffer = self.buffer.lock().unwrap();
        buffer.empty()
    }

    pub fn len(&self) -> usize {
        let buffer = self.buffer.lock().unwrap();
        buffer.len()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let buffer = self.buffer.lock().unwrap();
        buffer.to_vec()
    }

    pub fn pop_bytes(&self, num: usize) -> usize {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.pop_bytes(num)
    }

    pub fn resize(&self, new_size: usize) {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.resize(new_size);
    }

    pub fn write(&self, buf: &[u8]) -> std::io::Result<usize> {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.write(buf)
    }

    pub fn read(&self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.read(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_sync_ringbuffer_with_capacity() {
        let ruffer = SyncRingBuffer::with_capacity(1024);
        assert_eq!(ruffer.empty(), true);
        assert_eq!(ruffer.len(), 0);
        assert_eq!(ruffer.capacity(), 1024);
    }

    #[test]
    fn read_and_write_with_no_mut() {
        let ruffer = SyncRingBuffer::with_capacity(4);
        let write_data = "thisisatestofthesynccharacteristicofthesyncringbuffer".as_bytes();
        let read_data = &mut [0u8; 4];

        assert!(ruffer.write(write_data).is_ok());
        assert!(ruffer.read(read_data).is_ok());
        assert_eq!(read_data, &write_data[49..])
    }
}
