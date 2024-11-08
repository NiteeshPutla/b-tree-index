

## How BTreeMap is Used in This Project

This project utilizes Rust’s `BTreeMap` and leverages its efficient data handling capabilities to create a high-performance data indexing service.

### Data Storage
- **Efficient Operations**: `BTreeMap` is used as the core data structure for managing the dataset. It supports efficient insertion, deletion, and lookup, making it suitable for handling large volumes of sorted data.

### Concurrency
- **Safe Concurrent Access**: We employ `Arc` (Atomic Reference Counting) and `RwLock` (Read-Write Lock) to enable thread-safe access to the shared `BTreeMap`. This ensures that multiple readers can access the data concurrently, while write operations are safely locked to prevent data races.

### RESTful API
- **Simplified API Creation**: The `warp` framework is used to build a RESTful API around the `BTreeMap`. `warp` provides a clean and ergonomic way to define routes, handle HTTP requests, and integrate data operations seamlessly.

### Memory Management
- **Controlled Access**: Using `Arc` and `RwLock` ensures efficient and controlled memory management. By limiting data access through locking mechanisms, we minimize the risk of race conditions while keeping performance high.

### Efficient Queries
- **Optimized for Range Queries**: `BTreeMap` is particularly well-suited for scenarios requiring sorted data access and range queries. This feature aligns perfectly with our project's need for efficient data retrieval and database-level performance.

