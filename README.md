# ruffer 1.0.1
R[ing B]uffer is a simple overwriting ring buffer implementation.
A RingBuffer allocates it's memory once at creation on the heap.
The RingBuffer implements std::io::Read and std::io::Write for interacting with the buffer.
Any size buffer can be written to the RingBuffer, just note that only the capacity of the RingBuffer will be retained.
Reading data from the buffer will move the tail index, so the read data is essentially dropped.
If one wants to get a copy of the data on the form of a vector, a helper function are available to easily acquire one.

## Features
- `sync` - A Sync implementation of the RingBuffer.

## Usage
### Create a new RingBuffer with a specific capacity
```rust
use ruffer::RingBuffer;

let buffer = RingBuffer::with_capacity(1024);
```
### Write data to the buffer
```rust
use ruffer::RingBuffer;
use std::io::Write;

let mut buffer = RingBuffer::with_capacity(1024);
let write_data = "Test data buffer".as_bytes();
match buffer.write(&write_data) {
  Ok(bytes) => {
    println!("wrote {} bytes to buffer", bytes);
  }
  Err(e) => {
    println!("{}", e);
  }
}
```
### Read data from the buffer
```rust
use ruffer::RingBuffer;
use std::io::Read;

let mut buffer = RingBuffer::with_capacity(1024);
// ... use ringbuffer ...
let read_data = &mut [0u8; 32];
match buffer.read(read_data) {
  Ok(bytes) => {
    println!("read {} bytes from buffer", bytes);
  }
  Err(e) => {
    println!("{}", e);
  }
}
```

## License
[Apache-2.0](https://github.com/PrintPractical/ruffer/blob/main/LICENSE)
