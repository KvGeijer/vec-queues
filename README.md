# Vector Queues in Rust

This repository contains a variety of different queue designs in Rust. They are all implemented on top of vectors/arrays to have good memory performance. Furthermore, the designs try to not allocate unnecessary memory (e.g. wrapping the buffer instead of allocating new ones).

To support multiple designs, the crate also exports generic traits for different queue types.

## Queue designs

So far, the only queue design is the `ReallocatingQueue`, which is accessible through the `ReallocatingFifo` and `ReallocatingDeque` structs (with both wrap the core queue at [./src/reallocating_queues/internal_queue.rs](./src/reallocating_queues/internal_queue.rs)). This queue is built to not take up too much memory while at the same time providing amortized O(1) operations with unbounded size. Here are some key points:
- Does not use any unsafe, and builds upon a normal `Vec`.
- Uses circular wrapping to re-use memory instead of allocating unnecessarily.
- Re-allocates the buffer when full (leading to a single O(n) operation).

## Tests

There are tests inspired by property-based testing in the [tests/](./tests/) folder. These match the output of the queues in different scenarios with that of each other, or naive and inefficient wrappers around `Vec`. The tests are all generic over different queue implementations.

## Benchmarking

TODO. Should add benchmarks to compare the performance of different queues (for example, can we have a queue similar to the lcrq and compare its performance to the re-allocating one).
