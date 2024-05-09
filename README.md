# Vector Queues in Rust

This repository contains a variety of different queue designs in Rust. They are all implemented on top of vectors/arrays to have good memory performance. Furthermore, the designs try to not allocate unnecessary memory (e.g. wrapping the buffer instead of allocating new ones).

To support multiple designs, the crate also exports generic traits for different queue types.