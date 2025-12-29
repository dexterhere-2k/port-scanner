### Rust Multi-Threaded Port Scanner
A minimalist, high-performance port scanner that uses Rust's native threading to scan 1024 ports in under one second.

### Features
Concurrency: Spawns 1024 threads to scan ports in parallel.

Speed: Reduces scan time from minutes to milliseconds.

Safety: Uses Rust's ownership model and MPSC channels to prevent data races.
