

```text
                    PRODUCTS
       Blockchain / AI / Maps / Apps
                           │
                ─────────────────────
                DISTRIBUTED SYSTEMS
                ─────────────────────
                NETWORKING
                ─────────────────────
                DATABASES / STORAGE
                ─────────────────────
                OPERATING SYSTEMS
                ─────────────────────
                CONCURRENCY / PARALLELISM
                ─────────────────────
                COMPUTER ARCHITECTURE
                ─────────────────────
                DATA STRUCTURES / ALGORITHMS
                ─────────────────────
                PROGRAMMING LANGUAGES
                ─────────────────────
                MATHEMATICS
                ─────────────────────
                    PHYSICS / LOGIC
```

---

# 1. Rust — your primary language


### Language fundamentals


- variables
- mutability
- scalar types
- compound types
- functions
- expressions
- control flow
- modules
- crates
- packages
- Cargo
- visibility
- `struct`
- `enum`
- `match`
- pattern matching
- `Option`
- `Result`
- error propagation
- `if let`
- `while let`

### Ownership system

This is the heart of Rust.

- ownership
- moves
- copies
- borrowing
- mutable borrowing
- references
- slices
- lifetimes
- lifetime elision
- lifetime annotations
- `'static`
- borrowing rules
- ownership across functions
- ownership inside structs
- ownership in collections



### Types

- generics
- traits
- trait bounds
- associated types
- associated constants
- default trait methods
- blanket implementations
- `impl Trait`
- `dyn Trait`
- trait objects
- static vs dynamic dispatch
- `where`
- type aliases
- newtype pattern
- phantom types
- `Send`
- `Sync`

### Collections

Know the internals and tradeoffs of:

- `Vec`
- `VecDeque`
- `LinkedList`
- `HashMap`
- `HashSet`
- `BTreeMap`
- `BTreeSet`
- arrays
- slices
- strings
- `String`
- `&str`
- `Bytes`

And understand:

**Why would I choose this data structure?**

Not just how to write it.

---

# 2. Rust error handling

Learn to design failure rather than merely handle exceptions.

- `Result`
- `Option`
- `?`
- `unwrap`
- `expect`
- custom errors
- error enums
- `thiserror`
- `anyhow`
- error propagation
- error boundaries
- recoverable vs unrecoverable errors
- logging
- tracing
- graceful shutdown

For production systems, this matters enormously.

---

# 3. Iterators + functional Rust

Master:

- iterators
- `iter`
- `iter_mut`
- `into_iter`
- `map`
- `filter`
- `fold`
- `reduce`
- `collect`
- `find`
- `position`
- `any`
- `all`
- `enumerate`
- `zip`
- `chain`
- closures
- `move`
- function pointers
- iterator adapters

Goal:

```text
imperative thinking
        ↓
data transformation thinking
```

---

# 4. Async Rust


Learn:

- Futures
- `async`
- `.await`
- Tokio
- runtime
- tasks
- `spawn`
- `JoinHandle`
- `JoinSet`
- channels
- `select!`
- timers
- cancellation
- cancellation safety
- `Pin`
- `Poll`
- `Waker`
- executors
- cooperative scheduling
- blocking vs non-blocking
- `spawn_blocking`

Eventually understand what happens underneath:

```rust
some_future.await
```

You don't need to implement Tokio tomorrow.

But you should understand **what a Future actually is**.

---

# 5. Concurrency

Learn:

- threads
- shared state
- message passing
- channels
- mutexes
- `Mutex`
- `RwLock`
- atomics
- `Arc`
- `Rc`
- `Weak`
- `Send`
- `Sync`
- race conditions
- deadlocks
- starvation
- livelocks
- lock contention
- lock-free concepts
- memory ordering
- atomics

Then:

**concurrency ≠ parallelism**

Understand the difference deeply.

---

# 6. Parallelism

Learn:

- CPU cores
- processes
- threads
- work partitioning
- task parallelism
- data parallelism
- SIMD
- cache locality
- false sharing
- thread pools
- CPU affinity
- batching
- pipelining

This becomes extremely relevant for:

- market-data processing
- PBF processing
- routing
- embeddings
- inference
- blockchain validation

---

# 7. Computer architecture

This is where you stop being merely a framework programmer.

Learn:

- CPU
- ALU
- registers
- instruction cycle
- machine instructions
- assembly basics
- memory hierarchy
- RAM
- cache
- L1/L2/L3
- cache lines
- branch prediction
- virtual memory
- pages
- TLB
- memory-mapped I/O
- interrupts
- DMA
- buses
- storage
- SSD
- HDD
- NUMA
- endianness

Learn enough C/assembly to understand what Rust eventually becomes.

---

# 8. Operating systems

Learn:

- processes
- threads
- scheduling
- context switching
- system calls
- virtual memory
- page tables
- filesystems
- file descriptors
- sockets
- signals
- pipes
- IPC
- permissions
- kernel/user space
- memory allocation
- interrupts
- synchronization

Then build:

**your own tiny shell.**

Later:

**your own tiny OS/kernel experiments.**

---

# 9. Data structures

You need these cold.

### Linear

- arrays
- linked lists
- stacks
- queues
- deques

### Trees

- binary trees
- BST
- AVL
- red-black trees
- heaps
- tries
- B-trees

### Hashing

- hash tables
- collision resolution
- load factor
- hash functions

### Graphs

- adjacency list
- adjacency matrix
- BFS
- DFS
- DAG
- topological sort
- weighted graphs

---

# 10. Algorithms

Learn:

### Complexity

- Big-O
- Big-Theta
- Big-Omega
- amortized complexity
- time/space tradeoffs

### Searching

- linear search
- binary search
- hash lookup

### Sorting

- insertion
- selection
- merge
- quicksort
- heap sort
- counting/radix basics

### Graph algorithms

- BFS
- DFS
- Dijkstra
- Bellman-Ford
- Floyd-Warshall
- A\*
- topological sorting
- minimum spanning tree

### Problem solving

- recursion
- backtracking
- divide & conquer
- greedy
- dynamic programming
- sliding window
- two pointers
- prefix sums
- binary search on answer

---

# 11. Networking

This is mandatory if you want to build serious servers.

Understand the stack:

```text
HTTP
TLS
TCP
IP
Ethernet
```

Learn:

- packets
- IP
- MAC
- routing
- ports
- DNS
- DHCP
- ARP
- TCP
- UDP
- congestion control
- flow control
- sockets
- HTTP/1.1
- HTTP/2
- HTTP/3
- QUIC
- TLS
- WebSockets
- gRPC
- proxies
- load balancers
- NAT
- CDN

Then build:

**a TCP server in Rust.**

Then:

**an HTTP server in Rust.**

Then understand what Axum is actually giving you.

---

# 12. Backend engineering

Then master:

### Axum

- routers
- extractors
- middleware
- state
- handlers
- layers
- authentication
- authorization
- WebSockets
- error handling
- graceful shutdown

### Database

PostgreSQL:

- SQL
- schema design
- indexes
- B-tree indexes
- transactions
- ACID
- isolation levels
- locks
- MVCC
- query planning
- joins
- normalization
- denormalization
- connection pools
- migrations

### Rust database layer

- SQLx
- async database access
- transactions
- connection pooling
- prepared queries

---

# 13. Distributed systems

This is the level where your blockchain and billion-user ambitions become relevant.

Learn:

- replication
- sharding
- partitioning
- consistency
- availability
- latency
- CAP theorem
- quorum
- leader election
- consensus
- distributed locks
- distributed transactions
- retries
- idempotency
- timeouts
- backpressure
- event-driven architecture
- message queues
- pub/sub
- logs
- consensus algorithms
- Raft
- Paxos conceptually
- eventual consistency
- strong consistency
- fault tolerance
- Byzantine faults

---

# 14. Database/storage internals

Eventually build:

- key-value store
- WAL
- LSM tree
- SSTables
- memtable
- compaction
- B-tree storage
- indexing engine
- caching layer

This will dramatically improve your understanding of PostgreSQL, Redis, etc.

---

# 15. Cryptography

Required for your blockchain ambitions.

Learn:

- hashing
- SHA-2
- SHA-3
- HMAC
- random numbers
- entropy
- symmetric encryption
- AES
- asymmetric cryptography
- RSA conceptually
- elliptic curves
- Ed25519
- digital signatures
- public/private keys
- key derivation
- Merkle trees
- commitments
- zero-knowledge concepts

Then:

**build a toy blockchain.**

Not a cryptocurrency first.

---

# 16. Blockchain

Only after the foundations.

Understand:

```text
Transaction
     ↓
Signature
     ↓
Mempool
     ↓
Block
     ↓
Consensus
     ↓
Validation
     ↓
State
     ↓
Chain
```

Learn:

- transaction model
- UTXO
- account model
- blocks
- block headers
- Merkle trees
- wallets
- signatures
- nodes
- peer-to-peer networking
- mempool
- forks
- finality
- consensus
- PoW
- PoS
- Byzantine Fault Tolerance
- validator economics
- Sybil resistance
- smart contracts
- virtual machines



---

# 17. AI engineering

Later.

Learn:

### Mathematics

- linear algebra
- vectors
- matrices
- probability
- statistics
- derivatives
- gradients
- optimization

### ML

- regression
- classification
- loss functions
- gradient descent
- neural networks
- backpropagation
- embeddings
- attention
- Transformers
- inference
- quantization
- batching

### Systems

- model serving
- KV cache
- GPU basics
- CPU inference
- memory bandwidth
- quantization
- batching
- latency
- throughput


---

# 18. Frontend

You don't need to become a React specialist first.

Understand:

- HTML
- CSS
- JavaScript
- TypeScript
- browser architecture
- DOM
- HTTP
- WebSockets
- browser storage
- authentication
- rendering
- accessibility

Then:

- React
- Next.js

Eventually understand how the browser communicates with your Rust backend.

---

# 19. Mobile

Later:

- Android/iOS fundamentals
- networking
- local storage
- permissions
- background execution
- push notifications
- cryptography
- offline-first architecture

Rust can become your core engine/shared library, while the UI layer can remain platform-specific.

---

# 20. Security

Absolutely mandatory.

Learn:

- authentication
- authorization
- sessions
- JWT
- OAuth
- PKCE
- TLS
- password hashing
- secrets management
- SQL injection
- XSS
- CSRF
- SSRF
- replay attacks
- rate limiting
- DDoS concepts
- secure cookies
- cryptographic key management
- supply-chain security

---

# 21. Production engineering

A billion users don't care how elegant your code looks if it goes down.

Learn:

- Linux
- Docker
- CI/CD
- AWS
- networking
- observability
- metrics
- logs
- tracing
- alerting
- profiling
- load testing
- benchmarking
- capacity planning
- autoscaling
- caching
- CDN
- queues
- disaster recovery
- backups
- deployment strategies
- blue/green deployment
- canary deployment

---
