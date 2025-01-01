# ruffer 1.0.0
R[ing B]uffer is a simple overwriting ring buffer implementation.
A RingBuffer allocates it's memory once at creation on the heap.
The RingBuffer implements std::io::Read and std::io::Write for interacting with the buffer.
Any size buffer can be written to the RingBuffer, just note that only the capacity of the RingBuffer will be retained.
Reading data from the buffer will move the tail index, so the read data is essentially dropped.
If one wants to get a copy of the data on the form of a vector, a helper function are available to easily acquire one.

## Features
- `sync` - A Sync implementation of the RingBuffer.

## License
[Apache-2.0](https://github.com/PrintPractical/ruffer/blob/main/LICENSE)
