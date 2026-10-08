# MASTER SYSTEMS ENGINEERING & RUST ROADMAP

> Goal:
> Become an engineer capable of understanding, designing, implementing,
> optimizing, debugging and scaling complete computer systems from first principles.
>
> Primary language: Rust  
> Supporting languages: C, Assembly, SQL, TypeScript
>
> Core philosophy:
> Learn the abstraction.  
> Understand what it hides.  
> Rebuild a smaller version.  
> Measure it.  
> Break it.  
> Understand why it failed.  
> Then use the production abstraction.

***

# 0. ENGINEERING MINDSET

## 0.1 First Principles Thinking

- Define the actual problem
- Identify constraints
- Separate requirements from assumptions
- Reduce problems to primitives
- Eliminate unnecessary components
- Question every abstraction
- Question every dependency
- Understand causality
- Think in trade-offs
- Think in bottlenecks
- Think in invariants
- Think in failure modes

## 0.2 Systems Thinking [DONE]

- State
- Inputs
- Outputs
- Transformations
- Dependencies
- Resources
- Constraints
- Failure modes
- Feedback loops
- Observability
- Recovery

## 0.3 Engineering Trade-offs [DONE]

- Latency vs throughput
- Memory vs CPU
- Storage vs computation
- Consistency vs availability
- Simplicity vs flexibility
- Reliability vs cost
- Generality vs specialization
- Abstraction vs control
- Development speed vs runtime efficiency

## 0.4 Engineering Habits

- Read documentation
- Read source code
- Read RFCs
- Read technical papers
- Read specifications
- Debug from first principles
- Benchmark before optimizing
- Profile before guessing
- Write minimal reproductions
- Build from scratch
- Test assumptions
- Document decisions

# 1. PHYSICS & LOGIC

## 1.1 Physics Fundamentals

- Matter
- Energy
- Force
- Motion
- Electricity
- Voltage
- Current
- Resistance
- Power
- Electromagnetism
- Signals
- Frequency
- Waves

## 1.2 Digital Logic

- Boolean algebra
- AND
- OR
- NOT
- XOR
- NAND
- NOR
- Truth tables
- Logic gates
- Combinational logic
- Sequential logic
- Flip-flops
- Registers
- Counters
- Multiplexers
- Adders
- ALU
- Finite-state machines

## 1.3 Information Representation

- Bits
- Bytes
- Binary
- Decimal
- Hexadecimal
- Bitwise operations
- Bit masks
- Bit packing
- Signed integers
- Unsigned integers
- Two's complement
- Integer overflow
- Fixed-point representation
- Floating-point representation
- IEEE-754
- NaN
- Infinity
- Precision
- Rounding
- Endianness

# 2. MATHEMATICS

## 2.1 Discrete Mathematics

- Sets
- Relations
- Functions
- Logic
- Propositions
- Proofs
- Induction
- Combinatorics
- Permutations
- Combinations
- Graph theory
- Trees
- Recurrence relations

## 2.2 Algebra

- Algebraic manipulation
- Equations
- Inequalities
- Functions
- Exponents
- Logarithms

## 2.3 Linear Algebra

- Scalars
- Vectors
- Matrices
- Matrix multiplication
- Dot products
- Cross products
- Linear transformations
- Eigenvalues
- Eigenvectors
- Vector spaces

## 2.4 Calculus

- Limits
- Derivatives
- Partial derivatives
- Integrals
- Gradients
- Optimization

## 2.5 Probability

- Probability
- Conditional probability
- Bayes theorem
- Random variables
- Expected value
- Variance
- Standard deviation
- Probability distributions
- Normal distribution
- Bernoulli distribution
- Poisson distribution
- Markov chains

## 2.6 Statistics

- Sampling
- Mean
- Median
- Variance
- Correlation
- Regression
- Confidence intervals
- Hypothesis testing

## 2.7 Systems Mathematics

- Graph theory
- Queueing theory
- Little's Law
- Information theory
- Entropy
- Complexity theory

# 3. COMPUTER FUNDAMENTALS

## 3.1 Data Representation

- Binary
- Hexadecimal
- Bytes
- Memory addresses
- Integer representation
- Floating-point representation
- Character encoding
- ASCII
- Unicode
- UTF-8

## 3.2 Serialization

- Binary serialization
- Text serialization
- JSON
- MessagePack
- Protocol Buffers
- Serialization overhead
- Zero-copy serialization
- Versioned schemas
- Compatibility

## 3.3 Executables

- Object files
- Executables
- ELF
- Sections
- Symbols
- Relocations
- Static linking
- Dynamic linking
- Shared libraries
- Loaders
- Debug symbols

# 4. C PROGRAMMING

## 4.1 C Fundamentals

- Variables
- Types
- Functions
- Structs
- Enums
- Unions
- Preprocessor
- Header files
- Compilation

## 4.2 Memory

- Pointers
- Pointer arithmetic
- References through pointers
- Stack
- Heap
- malloc
- calloc
- realloc
- free
- Memory ownership
- Buffer management

## 4.3 Low-Level C

- Function pointers
- Struct layout
- Alignment
- Padding
- Undefined behavior
- Strict aliasing
- Volatile
- Atomics
- Memory ordering
- ABI
- FFI

## 4.4 C Toolchain

- GCC
- Clang
- Preprocessor
- Compiler
- Assembler
- Linker
- Debugger
- GDB

# 5. ASSEMBLY & MACHINE CODE

## 5.1 Assembly Fundamentals

- Registers
- Instruction pointer
- Stack pointer
- Flags
- Instructions
- Loads
- Stores
- Arithmetic
- Comparisons
- Branches
- Jumps
- Function calls
- Returns

## 5.2 Calling Conventions

- Stack frames
- Arguments
- Return values
- Register conventions
- ABI
- Stack alignment

## 5.3 CPU-Level Programming

- Atomics
- Compare-and-swap
- Memory barriers
- SIMD
- Intrinsics
- Syscalls

## 5.4 Architecture

- x86-64
- ARM64
- Instruction sets
- RISC vs CISC
- Machine code

## 5.5 Tooling

- objdump
- nm
- readelf
- gdb
- lldb
- perf

# 6. RUST — PRIMARY LANGUAGE

## 6.1 Language Fundamentals

- Variables
- Mutability
- Scalar types
- Compound types
- Functions
- Expressions
- Statements
- Control flow
- Modules
- Crates
- Packages
- Cargo
- Visibility
- Structs
- Enums
- Match
- Pattern matching
- Option
- Result
- if let
- while let

## 6.2 Ownership

- Ownership
- Moves
- Copies
- Borrowing
- Mutable borrowing
- References
- Slices
- Lifetimes
- Lifetime elision
- Lifetime annotations
- `'static`
- Borrow checker
- Ownership across functions
- Ownership inside structs
- Ownership in collections

## 6.3 Types

- Generics
- Traits
- Trait bounds
- Associated types
- Associated constants
- Default trait methods
- Blanket implementations
- impl Trait
- dyn Trait
- Trait objects
- Static dispatch
- Dynamic dispatch
- where clauses
- Type aliases
- Newtype pattern
- Phantom types
- Marker types
- Send
- Sync

## 6.4 Memory

- Stack
- Heap
- Box
- Rc
- Arc
- Weak
- Cell
- RefCell
- Mutex
- RwLock
- MaybeUninit
- ManuallyDrop
- Unsafe
- Raw pointers
- Memory layout

## 6.5 Collections

- Vec
- VecDeque
- LinkedList
- HashMap
- HashSet
- BTreeMap
- BTreeSet
- Arrays
- Slices
- String
- &str
- Bytes

For every collection:

- Internal representation
- Complexity
- Allocation behavior
- Cache locality
- Memory overhead
- Use cases
- Trade-offs

## 6.6 Error Handling

- Result
- Option
- ?
- unwrap
- expect
- Custom errors
- Error enums
- thiserror
- anyhow
- Error propagation
- Error boundaries
- Recoverable errors
- Unrecoverable errors
- Logging
- Tracing
- Graceful shutdown

## 6.7 Iterators & Functional Rust

- Iterator
- iter
- iter_mut
- into_iter
- map
- filter
- fold
- reduce
- collect
- find
- position
- any
- all
- enumerate
- zip
- chain
- Closures
- move closures
- Function pointers
- Iterator adapters

## 6.8 Advanced Rust

- Macros
- Declarative macros
- Procedural macros
- Derive macros
- Unsafe Rust
- FFI
- Pin
- Unpin
- Futures
- Async
- Generators/concepts
- Trait object internals
- Vtables
- Monomorphization
- Zero-cost abstractions
- Const generics
- Associated type bounds

## 6.9 Rust Tooling

- rustc
- Cargo
- rustfmt
- Clippy
- rust-analyzer
- cargo test
- cargo bench
- cargo flamegraph
- cargo audit
- cargo tree
- Miri
- Sanitizers
- Fuzzing

# 7. COMPILERS & LANGUAGE IMPLEMENTATION

## 7.1 Compiler Pipeline

```text
Source Code
    ↓
Lexer
    ↓
Parser
    ↓
AST
    ↓
Semantic Analysis
    ↓
Type Checking
    ↓
Intermediate Representation
    ↓
Optimization
    ↓
Code Generation
    ↓
Assembly
    ↓
Linker
    ↓
Binary
```

## 7.2 Compiler Concepts

- Lexing
- Parsing
- AST
- Symbol tables
- Scope
- Type systems
- Type checking
- Generic types
- Monomorphization
- Intermediate representation
- SSA
- Optimization
- Constant folding
- Dead-code elimination
- Inlining
- Register allocation
- Code generation

## 7.3 Toolchains

- LLVM
- LLVM IR
- rustc architecture
- Linkers
- Loaders
- ELF
- DWARF
- Debugging information

## 7.4 Build

- Tiny interpreter
- Tiny compiler
- Tiny programming language

# 8. DATA STRUCTURES

## 8.1 Linear

- Arrays
- Dynamic arrays
- Linked lists
- Stacks
- Queues
- Deques
- Ring buffers

## 8.2 Trees

- Binary trees
- BST
- AVL
- Red-black trees
- Heaps
- Tries
- B-trees
- B+ trees

## 8.3 Hashing

- Hash tables
- Hash functions
- Collision resolution
- Load factor
- Open addressing
- Chaining
- Consistent hashing

## 8.4 Graphs

- Graph representation
- Adjacency list
- Adjacency matrix
- Directed graphs
- Undirected graphs
- Weighted graphs
- DAGs
- BFS
- DFS

## 8.5 Specialized Structures

- Bloom filters
- Skip lists
- LRU cache
- LFU cache
- Union-Find
- Segment trees
- Fenwick trees
- Priority queues
- Sparse tables

# 9. ALGORITHMS

## 9.1 Complexity

- Big-O
- Big-Theta
- Big-Omega
- Amortized complexity
- Time complexity
- Space complexity
- Memory complexity
- Cache complexity

## 9.2 Searching

- Linear search
- Binary search
- Hash lookup
- Tree search

## 9.3 Sorting

- Insertion sort
- Selection sort
- Merge sort
- Quicksort
- Heapsort
- Counting sort
- Radix sort

## 9.4 Graph Algorithms

- BFS
- DFS
- Dijkstra
- Bellman-Ford
- Floyd-Warshall
- A*
- Topological sort
- Minimum spanning tree
- Kruskal
- Prim
- Strongly connected components
- Shortest paths
- Maximum flow

## 9.5 Problem Solving

- Recursion
- Backtracking
- Divide and conquer
- Greedy algorithms
- Dynamic programming
- Sliding window
- Two pointers
- Prefix sums
- Binary search on answer

## 9.6 Systems Algorithms

- Scheduling
- Caching
- Eviction
- Load balancing
- Rate limiting
- Consistent hashing
- Bloom filters
- Routing

# 10. MEMORY SYSTEMS

## 10.1 Memory Hierarchy

- Registers
- L1 cache
- L2 cache
- L3 cache
- RAM
- SSD
- HDD
- Remote storage

## 10.2 Memory Management

- Stack allocation
- Heap allocation
- malloc
- free
- Allocators
- Arena allocators
- Bump allocators
- Slab allocators
- Pool allocators
- Fragmentation
- Alignment
- Padding
- Object layout

## 10.3 Virtual Memory

- Virtual addresses
- Physical addresses
- Pages
- Page tables
- Multi-level page tables
- TLB
- Page faults
- Demand paging
- Copy-on-write
- Memory mapping
- mmap
- Shared memory

## 10.4 Advanced Memory

- Cache lines
- Cache locality
- False sharing
- NUMA
- Memory bandwidth
- Memory ordering
- Atomics
- DMA
- Zero-copy
- Memory-mapped files

# 11. COMPUTER ARCHITECTURE

## 11.1 CPU

- CPU
- ALU
- Registers
- Control unit
- Instruction cycle
- Machine instructions
- Instruction decoding

## 11.2 CPU Performance

- Pipelines
- Branch prediction
- Speculative execution
- Out-of-order execution
- Instruction-level parallelism
- SIMD
- Superscalar execution

## 11.3 Memory

- RAM
- Cache
- Cache hierarchy
- Cache lines
- TLB
- Memory controller
- Memory bandwidth
- NUMA

## 11.4 Hardware

- Buses
- PCIe
- USB
- NVMe
- SATA
- DMA
- Interrupts
- Network cards
- Storage controllers
- GPUs

# 12. OPERATING SYSTEMS

## 12.1 Processes

- Processes
- Threads
- Process lifecycle
- Scheduling
- Context switching
- CPU scheduling
- Priorities
- Signals

## 12.2 Kernel

- Kernel/user space
- System calls
- Interrupts
- Kernel scheduling
- Kernel memory
- Device drivers
- Kernel synchronization

## 12.3 IPC

- Pipes
- Unix sockets
- Shared memory
- Signals
- Message queues
- Semaphores

## 12.4 Filesystems

- Files
- Directories
- Inodes
- File descriptors
- Permissions
- Mounting
- Journaling
- Page cache
- VFS
- Filesystem consistency

## 12.5 Build

- Tiny shell
- Process manager
- Filesystem experiments
- Kernel experiments

# 13. LINUX INTERNALS

- Linux process model
- `/proc`
- `/proc/<pid>`
- Syscalls
- strace
- ltrace
- Signals
- File descriptors
- epoll
- io_uring
- mmap
- sendfile
- splice
- Pipes
- Unix sockets
- cgroups
- namespaces
- capabilities
- containers
- seccomp
- Linux networking
- Linux scheduler

# 14. I/O ARCHITECTURE

## 14.1 I/O Models

- Blocking I/O
- Non-blocking I/O
- Synchronous I/O
- Asynchronous I/O
- Event-driven I/O

## 14.2 Event Systems

- select
- poll
- epoll
- kqueue
- io_uring

## 14.3 Data Movement

- Buffering
- Scatter/gather I/O
- DMA
- Interrupt-driven I/O
- Zero-copy
- sendfile
- splice
- Memory mapping

## 14.4 Flow Control

- Backpressure
- Buffer limits
- Queueing
- Load shedding

# 15. CONCURRENCY

## 15.1 Fundamentals

- Threads
- Processes
- Shared state
- Message passing
- Channels

## 15.2 Synchronization

- Mutex
- RwLock
- Semaphore
- Condvar
- Barrier
- Atomics

## 15.3 Failure

- Race conditions
- Data races
- Deadlocks
- Starvation
- Livelocks
- Lock contention
- Priority inversion

## 15.4 Advanced Concurrency

- Lock-free algorithms
- Wait-free algorithms
- CAS
- Memory ordering
- Acquire
- Release
- Relaxed
- Sequential consistency
- Linearizability

> Concurrency ≠ parallelism.

# 16. PARALLELISM

- CPU cores
- Processes
- Threads
- Work partitioning
- Task parallelism
- Data parallelism
- SIMD
- Cache locality
- False sharing
- Thread pools
- CPU affinity
- Batching
- Pipelining
- Work stealing
- Parallel reductions

Applications:

- Market data
- PBF processing
- Routing
- Embeddings
- AI inference
- Blockchain validation

# 17. ASYNC RUST

- Futures
- async
- await
- Tokio
- Runtime
- Tasks
- spawn
- JoinHandle
- JoinSet
- Channels
- `select!`
- Timers
- Cancellation
- Cancellation safety
- Pin
- Poll
- Waker
- Executors
- Cooperative scheduling
- Blocking vs non-blocking
- spawn_blocking

Deep understanding:

```text
async function
      ↓
Future
      ↓
Poll
      ↓
Waker
      ↓
Executor
      ↓
Event loop
      ↓
OS I/O
```

# 18. NETWORKING

## 18.1 Fundamentals

- Bits over wire
- Packets
- Frames
- MAC addresses
- IP addresses
- Ports
- Routing
- MTU
- Fragmentation

## 18.2 Network Stack

```text
Application
    ↓
HTTP
    ↓
TLS
    ↓
TCP / QUIC / UDP
    ↓
IP
    ↓
Ethernet
```

## 18.3 TCP

- TCP lifecycle
- Three-way handshake
- Sequence numbers
- ACKs
- Retransmission
- RTT
- Congestion control
- Flow control
- Sliding window
- Nagle
- TIME_WAIT
- Connection reset
- Keep-alive

## 18.4 UDP

- Datagram model
- Reliability over UDP
- Ordering
- Retransmission
- Congestion handling

## 18.5 DNS

- DNS resolution
- Recursive resolvers
- Authoritative servers
- DNS records
- TTL
- DNS caching

## 18.6 Network Infrastructure

- DHCP
- ARP
- NAT
- Proxies
- Load balancers
- CDN
- Firewalls
- Reverse proxies

## 18.7 Modern Protocols

- HTTP/1.1
- HTTP/2
- HTTP/3
- QUIC
- TLS
- WebSockets
- gRPC

## 18.8 NAT Traversal

- STUN
- TURN
- ICE

## 18.9 Build

- TCP server
- HTTP server
- HTTP client
- WebSocket server
- UDP protocol

# 19. P2P NETWORKING

- Peer discovery
- Peer identity
- Peer connections
- Peer failure
- DHT
- Gossip
- Content addressing
- Chunking
- Parallel transfers
- Deduplication
- Peer reputation
- Routing
- NAT traversal
- Availability
- Replication
- Distributed discovery
- P2P caching
- Incentive mechanisms

Build:

```text
P2P file transfer system
        ↓
P2P messaging
        ↓
DHT
        ↓
Distributed content network
```

# 20. BACKEND ENGINEERING

## 20.1 HTTP Services

- Routing
- Middleware
- State
- Handlers
- Extractors
- Serialization
- Validation
- Authentication
- Authorization
- Rate limiting
- WebSockets
- Graceful shutdown

## 20.2 Axum

- Router
- Extractors
- Middleware
- State
- Layers
- Handlers
- Error handling
- WebSockets
- Authentication
- Authorization
- Graceful shutdown

## 20.3 API Design

- REST
- RPC
- gRPC
- Versioning
- Pagination
- Idempotency
- Rate limits
- API contracts
- Backward compatibility

# 21. DATABASES

## 21.1 SQL

- SELECT
- INSERT
- UPDATE
- DELETE
- JOIN
- GROUP BY
- Aggregation
- Subqueries
- CTEs
- Window functions

## 21.2 Database Design

- Schema design
- Normalization
- Denormalization
- Constraints
- Foreign keys
- Primary keys
- Indexes

## 21.3 PostgreSQL

- B-tree indexes
- Transactions
- ACID
- Isolation levels
- Locks
- MVCC
- Query planner
- Query optimizer
- Connection pools
- Migrations
- WAL
- Vacuum
- Replication

## 21.4 Rust

- SQLx
- Async database access
- Transactions
- Connection pooling
- Prepared queries

# 22. STORAGE ENGINE INTERNALS

## 22.1 Storage

- Pages
- Page layout
- Buffer pools
- Page cache
- WAL
- fsync
- Durability
- Crash recovery
- Checksums
- Journaling

## 22.2 Storage Structures

- Key-value stores
- Memtables
- SSTables
- LSM trees
- Compaction
- B-trees
- B+ trees
- Indexes
- Bloom filters

## 22.3 Performance

- Read amplification
- Write amplification
- Space amplification
- Caching
- Sequential vs random I/O

## 22.4 Transactions

- MVCC internals
- Isolation
- Locking
- Recovery
- Garbage collection

Build:

```text
KV store
    ↓
WAL
    ↓
SSTable
    ↓
LSM tree
    ↓
B-tree engine
    ↓
Mini database
```

# 23. DISTRIBUTED SYSTEMS

## 23.1 Failure Models

- Crash failure
- Network partition
- Message loss
- Message duplication
- Message reordering
- Partial failure
- Split brain

## 23.2 Reliability

- Failure detection
- Heartbeats
- Timeouts
- Retries
- Exponential backoff
- Circuit breakers
- Load shedding
- Graceful degradation
- Idempotency

## 23.3 Distributed Data

- Replication
- Sharding
- Partitioning
- Consistency
- Availability
- Latency
- Quorum
- Read replicas
- Leader/follower

## 23.4 Consensus

- Consensus problem
- Leader election
- Raft
- Paxos conceptually
- Byzantine faults
- Byzantine Fault Tolerance

## 23.5 Distributed Coordination

- Distributed locks
- Leases
- Distributed transactions
- Event ordering
- Logical clocks
- Vector clocks
- Event sourcing

## 23.6 Distributed Architecture

- Event-driven architecture
- Message queues
- Pub/sub
- Logs
- Streams
- Backpressure
- Exactly-once vs at-least-once
- At-most-once delivery

## 23.7 Core Concepts

- CAP theorem
- Strong consistency
- Eventual consistency
- Fault tolerance
- Linearizability
- Availability

# 24. FORMAL METHODS & CORRECTNESS

## 24.1 Correctness

- Invariants
- Preconditions
- Postconditions
- State machines
- Finite-state machines
- Determinism
- Safety
- Liveness

## 24.2 Distributed Correctness

- Linearizability
- Serializability
- Consensus safety
- Consensus liveness
- Failure invariants

## 24.3 Verification

- Property-based testing
- Model checking
- Formal verification
- Symbolic reasoning

## 24.4 Rust Verification

- Unit tests
- Integration tests
- Property tests
- Fuzzing
- Miri
- Sanitizers
- Loom
- Concurrency testing

# 25. CRYPTOGRAPHY

## 25.1 Hashing

- Hash functions
- SHA-2
- SHA-3
- Collision resistance
- Preimage resistance
- HMAC

## 25.2 Randomness

- Random numbers
- Entropy
- CSPRNG
- Nonces

## 25.3 Symmetric Cryptography

- AES
- Modes of operation
- Authentication
- AEAD

## 25.4 Asymmetric Cryptography

- Public/private keys
- RSA
- Elliptic curves
- Ed25519
- Key exchange

## 25.5 Signatures

- Digital signatures
- Signing
- Verification
- Key derivation
- Certificates

## 25.6 Advanced

- Merkle trees
- Commitments
- Zero-knowledge concepts
- Threshold cryptography concepts

# 26. BLOCKCHAIN

## 26.1 Fundamentals

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

## 26.2 Architecture

- Transactions
- UTXO
- Account model
- Blocks
- Block headers
- Merkle trees
- Wallets
- Signatures
- Nodes
- P2P networking
- Mempool
- Forks
- Finality

## 26.3 Consensus

- Proof of Work
- Proof of Stake
- Byzantine Fault Tolerance
- Validator economics
- Sybil resistance

## 26.4 Execution

- Smart contracts
- Virtual machines
- State machines
- Gas
- Deterministic execution

Build:

- Toy blockchain
- Toy P2P blockchain
- Toy consensus algorithm

> Do not start with a cryptocurrency.

# 27. SECURITY

## 27.1 Application Security

- Authentication
- Authorization
- Sessions
- JWT
- OAuth
- PKCE
- TLS
- Password hashing
- Secrets management
- Secure cookies

## 27.2 Web Security

- SQL injection
- XSS
- CSRF
- SSRF
- Replay attacks
- Session attacks
- Rate limiting
- DDoS

## 27.3 Systems Security

- Memory safety
- Buffer overflows
- Use-after-free
- Race vulnerabilities
- Privilege escalation
- Sandboxing
- Capability security
- Secure boot

## 27.4 Advanced Security

- Timing attacks
- Cache attacks
- Side channels
- Supply-chain attacks
- Dependency attacks
- Key management
- Threat modeling
- Fuzzing

# 28. AI ENGINEERING

## 28.1 Mathematics

- Linear algebra
- Probability
- Statistics
- Derivatives
- Gradients
- Optimization

## 28.2 Machine Learning

- Regression
- Classification
- Loss functions
- Gradient descent
- Neural networks
- Backpropagation
- Embeddings
- Attention
- Transformers

## 28.3 AI Systems

- Inference
- Quantization
- Batching
- Model serving
- KV cache
- CPU inference
- GPU basics
- Memory bandwidth
- Latency
- Throughput

## 28.4 AI Infrastructure

- Model loading
- Tokenization
- Vector databases
- Retrieval
- RAG
- Embedding pipelines
- Model caching
- Distributed inference
- GPU scheduling

# 29. FRONTEND & BROWSER ENGINEERING

## 29.1 Web Fundamentals

- HTML
- CSS
- JavaScript
- TypeScript
- DOM
- HTTP
- WebSockets
- Browser storage
- Cookies
- Authentication
- Rendering
- Accessibility

## 29.2 Browser Architecture

- Browser process
- Renderer process
- JavaScript engine
- Event loop
- DOM
- Rendering pipeline
- Layout
- Paint
- Compositing
- GPU acceleration

## 29.3 Frameworks

- React
- Next.js

> Goal:  
> Understand how the browser communicates with the Rust backend rather than merely knowing framework APIs.

# 30. MOBILE

- Android fundamentals
- iOS fundamentals
- Networking
- Local storage
- Permissions
- Background execution
- Push notifications
- Cryptography
- Offline-first architecture
- Rust shared libraries
- Platform-specific UI

# 31. PERFORMANCE ENGINEERING

## 31.1 Benchmarking

- Benchmark design
- Microbenchmarks
- Macrobenchmarks
- Warm-up
- Variance
- Reproducibility

## 31.2 Profiling

- CPU profiling
- Memory profiling
- Allocation profiling
- Flame graphs
- perf
- Cache profiling
- I/O profiling

## 31.3 Metrics

- Latency
- Throughput
- p50
- p90
- p95
- p99
- p99.9
- Error rate
- Saturation

## 31.4 Queueing

- Queueing theory
- Little's Law
- Service rate
- Arrival rate
- Queue depth
- Tail latency

## 31.5 Optimization

- Cache locality
- Allocation reduction
- Batching
- Vectorization
- Parallelism
- Lock reduction
- Zero-copy
- Syscall reduction
- Network hop reduction

# 32. TESTING

- Unit testing
- Integration testing
- End-to-end testing
- Property testing
- Fuzz testing
- Load testing
- Stress testing
- Chaos testing
- Fault injection
- Deterministic testing
- Concurrency testing
- Race detection
- Reproducible testing

# 33. OBSERVABILITY

## 33.1 Metrics

- Counters
- Gauges
- Histograms
- Percentiles
- RED metrics
- USE metrics

## 33.2 Logging

- Structured logs
- Log levels
- Correlation IDs
- Request IDs
- Distributed tracing

## 33.3 Monitoring

- Health checks
- Readiness
- Liveness
- Alerts
- SLOs
- SLIs
- SLAs
- Error budgets

## 33.4 Debugging

- Core dumps
- Stack traces
- Distributed traces
- Profilers
- Production debugging

# 34. PRODUCTION ENGINEERING

## 34.1 Linux

- Linux
- Shell
- Processes
- Networking
- Filesystems
- Permissions

## 34.2 Containers

- Docker
- Container images
- Namespaces
- cgroups
- Container networking
- Container storage

## 34.3 Cloud

- AWS
- Compute
- Storage
- Networking
- Databases
- IAM
- Load balancers

## 34.4 CI/CD

- Git
- GitHub
- CI
- Automated tests
- Build pipelines
- Deployment pipelines
- Artifact management

## 34.5 Deployment

- Rolling deployment
- Blue/green deployment
- Canary deployment
- Feature flags
- Rollbacks

## 34.6 Reliability

- Capacity planning
- Autoscaling
- Caching
- CDN
- Queues
- Backups
- Disaster recovery
- Replication
- Multi-region systems

# 35. PRODUCTS & SYSTEM DESIGN

> Only after understanding the layers beneath them.

## 35.1 Architecture

- Requirements
- Constraints
- Architecture diagrams
- Component boundaries
- Interfaces
- Data flows
- Failure modes
- Capacity planning

## 35.2 Product Infrastructure

- Authentication
- Identity
- Payments
- Notifications
- Search
- Maps
- Routing
- Recommendations
- Analytics
- Event systems

## 35.3 Your Systems

### Koel

- Identity
- Protocol
- Decision engine
- Memory
- Learning systems
- Distributed protocol
- Trust
- Reputation
- P2P infrastructure

### Phunsuk

- Maps
- Places
- Observations
- Confirmation
- Consensus
- Trust weighting
- Travel infrastructure

### AlpKid

- Market data ingestion
- Streaming
- Event processing
- Time-series storage
- Low-latency decisions
- Risk engine
- Execution
- Backtesting
- Monitoring

### Sari

- Data ingestion
- Nutrition models
- Recommendation systems
- AI inference
- Privacy
- Identity
- Data security

# 36. CAPSTONE SYSTEMS PROJECTS

> Do not merely complete courses.  
> Build systems.

## Level 1 — Language

- CLI tools
- Parser
- Interpreter
- Mini compiler

## Level 2 — Memory

- Allocator
- Arena allocator
- LRU cache
- Ring buffer

## Level 3 — OS

- Shell
- Process manager
- File system experiments
- mmap-based storage

## Level 4 — Networking

- TCP server
- HTTP server
- HTTP client
- WebSocket server
- UDP protocol

## Level 5 — Storage

- KV store
- WAL
- SSTable
- LSM tree
- B-tree database

## Level 6 — Distributed Systems

- Distributed KV store
- Replicated database
- Raft
- Leader election
- Distributed queue

## Level 7 — P2P

- P2P messaging
- Peer discovery
- DHT
- P2P file sharing
- Distributed content network

## Level 8 — Blockchain

- Toy blockchain
- P2P blockchain
- Mempool
- Consensus
- Validator network

## Level 9 — AI Systems

- Embedding engine
- Vector search
- RAG
- Local inference server
- Distributed inference

## Level 10 — Real Products

- Koel Protocol
- KoelMaps
- Phunsuk
- AlpKid
- Sari

# 37. THE ENGINEERING LOOP

For every concept:

```text
1. UNDERSTAND
       ↓
2. REDUCE TO PRIMITIVES
       ↓
3. IMPLEMENT A SMALL VERSION
       ↓
4. TEST IT
       ↓
5. BENCHMARK IT
       ↓
6. BREAK IT
       ↓
7. DEBUG IT
       ↓
8. READ THE REAL IMPLEMENTATION
       ↓
9. OPTIMIZE IT
       ↓
10. USE IT IN A REAL SYSTEM
```

> Never learn only by consuming information.

# 38. FINAL ENGINEERING STANDARD

Eventually you should be able to look at a system and ask:

- What is the actual problem?
- What are the constraints?
- What state exists?
- Where does the state live?
- How does data move?
- How many bytes move?
- How many copies occur?
- How many allocations occur?
- How many syscalls occur?
- How many network hops occur?
- What is the latency?
- What is the throughput?
- What happens under load?
- What happens when a node dies?
- What happens when packets disappear?
- What happens when messages arrive twice?
- What happens when the database dies?
- What happens when the network partitions?
- What is the invariant?
- What can be eliminated?
- What can be cached?
- What can be parallelized?
- What can be made zero-copy?
- What can be made deterministic?
- What abstraction is hiding the real mechanism?
- Can I build a smaller version myself?
