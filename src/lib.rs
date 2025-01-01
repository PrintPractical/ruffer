//! R[ing B]uffer is a simple overwriting ring buffer implementation.
//! A RingBuffer allocates it's memory once at creation on the heap.
//! The RingBuffer implements std::io::Read and std::io::Write for interacting with the buffer.
//! Any size buffer can be written to the RingBuffer, just note that only the capacity of the RingBuffer will be retained.
//! Reading data from the buffer will move the tail index, so the read data is essentially dropped.
//! If one wants to get a copy of the data on the form of a vector, a helper function are available to easily acquire one.
//! 
//! # Features
//! - `sync` - A Sync implementation of the RingBuffer.

#[cfg(feature = "sync")]
pub mod sync;

const DEFAULT_CAPACITY: usize = 10240;

pub struct RingBuffer {
    buffer: Vec<u8>,
    capacity: usize,
    len: usize,
    head: usize,
    tail: usize,
}

// Static Impls
impl RingBuffer {
    pub fn new() -> Self {
        RingBuffer::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(size: usize) -> Self {
        RingBuffer {
            buffer: vec![0u8; size],
            capacity: size,
            len: 0,
            head: 0,
            tail: 0,
        }
    }
}

// Member Impls
impl RingBuffer {
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut ret = vec![0u8; self.len];
        let slice = ret.as_mut_slice();
        for i in 0..self.len {
            slice[i] = self.buffer[(self.tail + i) % self.capacity]
        }
        ret
    }

    pub fn pop_bytes(&mut self, num: usize) -> usize {
        let actual_num = std::cmp::min(self.len, num);
        self.len -= actual_num;
        self.tail = (self.tail + actual_num) % self.capacity;
        actual_num
    }

    pub fn resize(&mut self, new_size: usize) {
        if self.capacity != new_size {
            if self.len > new_size {
                self.pop_bytes(self.len - new_size);
            }
            self.buffer = self.to_vec();
            self.len = self.buffer.len();
            self.head = self.len % new_size;
            self.tail = 0;
            self.capacity = new_size;
        }
    }
}

impl std::io::Write for RingBuffer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let buffer = self.buffer.as_mut_slice();
        for i in 0..buf.len() {
            buffer[self.head] = buf[i];
            if self.head == self.tail && self.len > 0 {
                self.tail = (self.tail + 1) % self.capacity;
            } else {
                self.len += 1;
            }
            self.head = (self.head + 1) % self.capacity;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl std::io::Read for RingBuffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut bytes = 0;
        if self.len != 0 {
            let buffer = self.buffer.as_slice();
            bytes = std::cmp::min(self.len, buf.len());
            for i in 0..bytes {
                buf[i] = buffer[self.tail];
                self.tail = (self.tail + 1) % self.capacity;
                self.len -= 1;
            }
        }
        Ok(bytes)
    }
}

impl Default for RingBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    use super::*;

    #[test]
    fn create_ringbuffer_default() {
        let ruffer = RingBuffer::new();
        assert_eq!(ruffer.capacity(), DEFAULT_CAPACITY);
    }

    #[test]
    fn create_ringbuffer_with_capacity() {
        let ruffer = RingBuffer::with_capacity(1024);
        assert_eq!(ruffer.capacity(), 1024);
    }

    #[test]
    fn is_empty() {
        let ruffer = RingBuffer::with_capacity(1024);
        assert!(ruffer.empty());
    }

    #[test]
    fn to_vec_zero_tail() {
        let mut ruffer = RingBuffer::with_capacity(16);
        let write_data = "data".as_bytes();
        assert!(ruffer.write(write_data).is_ok());
        let res = ruffer.to_vec();
        assert_eq!(res.len(), 4);
        assert_eq!(res, write_data.to_vec());
    }

    #[test]
    fn to_vec_nonzero_tail() {
        let mut ruffer = RingBuffer::with_capacity(4);
        let write_data = "thisisatest    data".as_bytes();
        assert!(ruffer.write(write_data).is_ok());
        let res = ruffer.to_vec();
        assert_eq!(res.len(), 4);
        assert_eq!(res, write_data[15..19].to_vec());
    }

    #[test]
    fn pop_bytes_nowrap() {
        let mut ruffer = RingBuffer::with_capacity(16);
        let write_data = "data".as_bytes();
        let read_data = &mut [0u8; 16];
        assert!(ruffer.write(write_data).is_ok());
        assert_eq!(ruffer.pop_bytes(2), 2);
        assert_eq!(ruffer.len(), 2);
        assert!(ruffer.read(read_data).is_ok());
        assert_eq!(read_data[0..2].to_vec(), write_data[2..4].to_vec())
    }

    #[test]
    fn pop_bytes_wrap() {
        let mut ruffer = RingBuffer::with_capacity(4);
        let write_data = "data123".as_bytes();
        let read_data = &mut [0u8; 16];
        assert!(ruffer.write(write_data).is_ok());
        assert_eq!(ruffer.pop_bytes(2), 2);
        assert_eq!(ruffer.len(), 2);
        assert!(ruffer.read(read_data).is_ok());
        assert_eq!(read_data[0..2].to_vec(), write_data[5..7].to_vec())
    }

    #[test]
    fn write_less_than_capacity() {
        let mut ruffer = RingBuffer::with_capacity(16);
        let data = [1u8; 5];
        let res = ruffer.write(&data);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 5);
        assert!(!ruffer.empty());
        assert_eq!(ruffer.len(), 5);
    }

    #[test]
    fn write_equal_to_capacity() {
        let mut ruffer = RingBuffer::with_capacity(16);
        let data = [1u8; 16];
        let res = ruffer.write(&data);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 16);
        assert!(!ruffer.empty());
        assert_eq!(ruffer.len(), 16);
    }

    #[test]
    fn write_greater_than_capacity() {
        let mut ruffer = RingBuffer::with_capacity(16);
        let data = [1u8; 32];
        let res = ruffer.write(&data);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 32);
        assert!(!ruffer.empty());
        assert_eq!(ruffer.len(), 16);
    }

    #[test]
    fn read_empty() {
        let mut ruffer = RingBuffer::with_capacity(16);
        let data = &mut [0u8; 16];
        let res = ruffer.read(data);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0);
    }

    #[test]
    fn write_less_than_capacity_then_read() {
        let mut ruffer = RingBuffer::with_capacity(16);
        let write_data = "test".as_bytes();
        let read_data = &mut [0u8; 16];

        assert!(ruffer.write(write_data).is_ok());
        let res = ruffer.read(read_data);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 4);
        assert_eq!(&read_data[0..4], write_data);
    }

    #[test]
    fn write_equal_to_capacity_then_read() {
        let mut ruffer = RingBuffer::with_capacity(4);
        let write_data = "test".as_bytes();
        let read_data = &mut [0u8; 16];

        assert!(ruffer.write(write_data).is_ok());
        let res = ruffer.read(read_data);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 4);
        assert_eq!(&read_data[0..4], write_data);
    }

    #[test]
    fn write_greater_than_capacity_then_read() {
        let mut ruffer = RingBuffer::with_capacity(4);
        let write_data = "testgreater".as_bytes();
        let read_data = &mut [0u8; 16];

        assert!(ruffer.write(write_data).is_ok());
        let res = ruffer.read(read_data);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 4);
        assert_eq!(&read_data[0..4], &write_data[7..11]);
    }

    #[test]
    fn resize_smaller_than_original() {
        let mut ruffer = RingBuffer::with_capacity(16);
        let write_data = "thisisatest".as_bytes();
        let read_data = &mut [0u8; 16];

        assert!(ruffer.write(write_data).is_ok());
        ruffer.resize(4);
        let res = ruffer.read(read_data);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 4);
        assert_eq!(&read_data[0..4], &write_data[7..11]);
    }

    #[test]
    fn resize_bigger_than_original() {
        let mut ruffer = RingBuffer::with_capacity(16);
        let write_data = "thisisatest".as_bytes();
        let read_data = &mut [0u8; 16];

        assert!(ruffer.write(write_data).is_ok());
        ruffer.resize(32);
        let res = ruffer.read(read_data);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 11);
        assert_eq!(&read_data[0..11], &write_data[0..11]);
    }
}