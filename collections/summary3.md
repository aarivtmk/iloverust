# Rust Collections — Final Mental Model

We now have:

```text
ARRAY
SLICE
String
&str
Bytes
```

---

# 1. Array `[T; N]`

An array has a **fixed size known at compile time**.

```rust
let numbers = [10, 20, 30, 40];
```

Memory:

```text
STACK

numbers
┌────┬────┬────┬────┐
│ 10 │ 20 │ 30 │ 40 │
└────┴────┴────┴────┘
```

For `i32`:

```text
4 × 4 = 16 bytes
```

The entire array is one contiguous block.

```rust
numbers[2] // 30
```

The CPU can calculate the address directly:

```text
address = base + index × element_size
```

Therefore:

```text
access → O(1)
```

### Key property

```text
[T; N]
```

means:

> **Exactly N elements.**

---

# 2. Slice `[T]` / `&[T]`

This is extremely important.

A slice is **not the actual storage**.

It is a **view into existing contiguous memory**.

```rust
let numbers = [10, 20, 30, 40];

let slice = &numbers[1..3];
```

Memory:

```text
ARRAY

[10][20][30][40]
     ↑       ↑
     └───────┘
       slice
```

Conceptually:

```text
slice
┌─────────────────┐
│ pointer         │
│ length = 2      │
└─────────────────┘
```

On a 64-bit system, a reference to a slice is commonly **16 bytes**:

```text
pointer = 8 bytes
length  = 8 bytes
```

The slice doesn't own the data.

```text
array owns memory
       ↓
   &[T] borrows it
```

That's why this works:

```rust
fn sum(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}
```

You can pass:

```rust
sum(&array);
```

or:

```rust
sum(&vec);
```

because both can be viewed as slices.

---

# 3. `String`

`String` is an **owned, growable UTF-8 string**.

```rust
let name = String::from("Aariv");
```

Conceptually:

```text
STACK

name
┌───────────────┐
│ ptr           │──────────────┐
│ len = 5       │              │
│ capacity = 5  │              │
└───────────────┘              │
                               ↓
HEAP

[ a ][ a ][ r ][ i ][ v ]
```

On a 64-bit system:

```text
pointer   = 8
length    = 8
capacity  = 8
----------------
             24 bytes
```

The characters themselves live in the heap.

And because Rust strings are UTF-8:

```rust
let s = String::from("é");
```

does **not** mean:

```text
length = 1 byte
```

`é` uses 2 UTF-8 bytes.

So:

```rust
s.len()
```

returns **bytes**, not number of human-visible characters.

---

# 4. `&str`

`&str` is a **borrowed string slice**.

```rust
let name = String::from("Aariv");

let part: &str = &name[0..3];
```

Memory:

```text
HEAP

[ a ][ a ][ r ][ i ][ v ]
  ↑
  │
  └──── &str
```

Conceptually:

```text
&str
┌────────────────┐
│ pointer        │
│ length         │
└────────────────┘
```

Again, commonly:

```text
16 bytes
```

on a 64-bit system.

It does **not own the string data**.

So:

```text
String
    ↓ owns
heap bytes

&str
    ↓ borrows
those bytes
```

This distinction is fundamental:

```text
String → OWNED
&str   → BORROWED
```

---

# 5. `String` vs `&str`

This is one of the most important Rust distinctions.

```text
String
┌─────────────────────────┐
│ ptr                     │
│ len                     │
│ capacity                │
└────────────┬────────────┘
             ↓
          HEAP DATA
       [a][a][r][i][v]
```

versus:

```text
&str
┌─────────────────────────┐
│ ptr                     │
│ len                     │
└────────────┬────────────┘
             ↓
       existing string data
```

### Think:

```text
String = "I own this."

&str = "I'm looking at this."
```

---

# 6. `Bytes`

Now we're entering the networking/backend world.

`Bytes` usually refers to the `bytes` crate:

```rust
use bytes::Bytes;
```

It's designed for **efficient handling of byte buffers**, especially in networking and async systems.

For example:

```rust
let data = Bytes::from("hello");
```

Conceptually:

```text
Bytes
┌────────────────────┐
│ pointer            │
│ length             │
│ shared/storage info│
└──────────┬─────────┘
           ↓
HEAP
[ h ][ e ][ l ][ l ][ o ]
```

The important difference from `Vec<u8>` is that `Bytes` is designed around **cheap sharing and slicing of byte data**.

For example:

```rust
let data = Bytes::from("hello world");

let part = data.slice(0..5);
```

Conceptually:

```text
original:

[ h ][ e ][ l ][ l ][ o ][ ][w][o][r][l][d]
  ↑
  │
  └──── original Bytes


part:

[ h ][ e ][ l ][ l ][ o ]
  ↑
  └──── points into the same underlying storage
```

You don't necessarily copy all five bytes.

That's extremely useful for:

```text
HTTP
TCP
WebSockets
network protocols
async servers
buffers
```

Which is why you'll encounter `Bytes` frequently when working with Rust backend/networking libraries.

---

# 7. The big memory picture

Now put everything together.

### Array

```text
[T; N]

STACK
[10][20][30][40]
```

Owns the data directly.

---

### Vec

```text
STACK
┌──────────────┐
│ ptr          │
│ len          │
│ capacity     │
└──────┬───────┘
       ↓
HEAP
[10][20][30][40]
```

Owns growable contiguous data.

---

### Slice

```text
STACK
┌──────────────┐
│ ptr ────────────────→ existing data
│ len          │
└──────────────┘
```

Borrows contiguous data.

---

### String

```text
STACK
┌──────────────┐
│ ptr          │
│ len          │
│ capacity     │
└──────┬───────┘
       ↓
HEAP
[UTF-8 bytes]
```

Owns growable UTF-8 data.

---

### `&str`

```text
STACK
┌──────────────┐
│ ptr ────────────────→ UTF-8 bytes
│ len          │
└──────────────┘
```

Borrows UTF-8 data.

---

### `Bytes`

```text
Bytes
   │
   ↓
byte storage
   │
   ├── cheap slices
   └── efficient sharing
```

Designed for byte-oriented systems/network programming.

---

# 8. Complexity

| Type            |                 Index |           Search |              Insert/remove |
| --------------- | --------------------: | ---------------: | -------------------------: |
| `[T; N]`        |                  O(1) |             O(n) |                 fixed size |
| `Vec<T>`        |                  O(1) |             O(n) |       back: O(1) amortized |
| `VecDeque<T>`   |                  O(1) |             O(n) | front/back: O(1) amortized |
| `LinkedList<T>` |                  O(n) |             O(n) |                 ends: O(1) |
| `HashMap<K,V>`  |                     — | **O(1) average** |           **O(1) average** |
| `BTreeMap<K,V>` |                     — |     **O(log n)** |               **O(log n)** |
| `BTreeSet<T>`   |                     — |     **O(log n)** |               **O(log n)** |
| `&[T]`          |                  O(1) |             O(n) |    cannot modify structure |
| `String`        | no character indexing |             O(n) |     append: O(1) amortized |
| `&str`          | no character indexing |             O(n) |              cannot modify |
| `Bytes`         |    O(1) byte indexing |             O(n) |      immutable byte buffer |

---

# 9. The collection decision tree

This is what I actually want you to remember.

```text
                    WHAT DO I NEED?
                         │
          ┌──────────────┴──────────────┐
          │                             │
     Fixed-size?                    Growable?
          │                             │
        ARRAY                           │
                                        ↓
                              ┌─────────┴─────────┐
                              │                   │
                         contiguous?          key/value?
                              │                   │
                             yes                  yes
                              │                   │
                             Vec             ┌────┴─────┐
                                             │          │
                                         unordered    ordered
                                             │          │
                                          HashMap    BTreeMap
```

For sets:

```text
Need unique values?
        │
        ├── unordered → HashSet
        │
        └── ordered   → BTreeSet
```

For queues:

```text
Need both ends?
      ↓
  VecDeque
```

For strings:

```text
Own the string?
      ↓
   String

Borrow string?
      ↓
    &str
```

For networking:

```text
raw bytes
   ↓
 Bytes
```

---

# 10. The deeper principle

You've now learned enough collections to start thinking like a systems programmer.

Don't ask:

> "Which collection is fastest?"

Ask:

> **"What operations does my program perform most frequently, and what memory layout makes those operations cheap?"**

For example, Koel's routing system might want:

```text
graph nodes
     ↓
Vec
```

because nodes can be indexed directly.

A user lookup:

```text
user_id → User
```

might use:

```text
HashMap
```

A sorted range of timestamps:

```text
timestamp → event
```

could use:

```text
BTreeMap
```

A unique sorted collection of IDs:

```text
BTreeSet
```

A network payload:

```text
HTTP/WebSocket bytes
       ↓
Bytes
```

That's the level you should operate at.

---

## Final checkpoint 🧠

Before moving on, make sure these statements feel obvious:

```text
[T; N]
    ↓
fixed contiguous storage

Vec<T>
    ↓
owned growable contiguous storage

&[T]
    ↓
borrowed view of contiguous storage

String
    ↓
owned growable UTF-8 bytes

&str
    ↓
borrowed UTF-8 bytes

Bytes
    ↓
efficient byte-oriented buffer/sharing
```

And the **most important ownership distinction**:

```text
OWN
 │
 ├── Vec
 ├── String
 └── Bytes (with its own ownership semantics)
 
BORROW
 │
 ├── &[T]
 └── &str
```

**Collections section: done.** The next level should be **iterators, `IntoIterator`, `Iterator`, `map`, `filter`, `fold`, `collect`, closures, and how Rust's iterator machinery actually works under the hood**.
