## First: `Array`

```rust
let a = [10u64, 20, 30];
```

An array is simply the elements themselves:

```text
Stack
┌────────┬────────┬────────┐
│ 10     │ 20     │ 30     │
│ 8 bytes│ 8 bytes│ 8 bytes│
└────────┴────────┴────────┘

Total = 24 bytes
```

Because:

```text
[u64; 3] = 3 × 8 = 24 bytes
```

No pointer, length, or capacity is required.

The type itself tells Rust:

```text
[u64; 3]
   ↑   ↑
 type  number of elements
```

---

# `Vec`

```rust
let v = vec![10u64, 20, 30];
```

The `Vec` variable itself is a small descriptor:

```text
Stack

v
┌──────────────┐
│ pointer      │ ──────────────┐
│ length = 3   │               │
│ capacity = 3 │               │
└──────────────┘               │
                               ↓
Heap
┌────────┬────────┬────────┐
│ 10     │ 20     │ 30     │
│ 8      │ 8      │ 8      │
└────────┴────────┴────────┘
```

On a typical 64-bit machine, the three `usize` fields are **8 bytes each**, so the `Vec` descriptor is commonly:

```text
pointer    = 8 bytes
length     = 8 bytes
capacity   = 8 bytes
--------------------
             24 bytes
```

And the heap allocation:

```text
3 × u64 = 24 bytes
```

So conceptually:

```text
Vec object:      24 bytes
Heap elements:   24 bytes
Total:           ~48 bytes
```

But don't treat that as a universal `Vec` memory-cost formula; allocator overhead and the actual element type matter.

---

# `VecDeque`

```rust
let q = VecDeque::from([10u64, 20, 30]);
```

Conceptually it also has a small descriptor on the stack and a heap allocation containing the elements.

Think:

```text
Stack
┌────────────────────┐
│ pointer            │
│ length             │
│ capacity           │
│ front/start index  │
└─────────┬──────────┘
          ↓
Heap
┌────────┬────────┬────────┬────────┐
│ 10     │ 20     │ 30     │ empty  │
└────────┴────────┴────────┴────────┘
```

The important extra concept is that the **logical front can move around the physical allocation**.

The exact internal representation should be treated as an implementation detail.

---

# `LinkedList`

This one is completely different.

```rust
let mut list = LinkedList::new();

list.push_back(10u64);
list.push_back(20u64);
list.push_back(30u64);
```

Conceptually:

```text
Stack

list
┌─────────────┐
│ head ───────┼──────────────┐
│ tail ───────┼──────────┐   │
└─────────────┘          │   │
                         │   │
                         ↓   ↓
Heap

      ┌─────────────────┐
      │ value = 10      │
      │ prev = ...      │
      │ next ───────────┼──────┐
      └─────────────────┘      │
                               ↓
                       ┌─────────────────┐
                       │ value = 20      │
                       │ prev            │
                       │ next ───────────┼──────┐
                       └─────────────────┘      │
                                               ↓
                                       ┌─────────────────┐
                                       │ value = 30      │
                                       │ prev            │
                                       │ next            │
                                       └─────────────────┘
```

Each node has:

```text
value
prev pointer
next pointer
```

plus allocation overhead.

So a `LinkedList<u64>` with 3 elements has:

```text
3 × (value + pointers + allocator/node overhead)
```

It's significantly more memory-hungry than a `Vec<u64>`.

And again, **the exact byte size is implementation/allocator dependent**.

---

# Now the important one: `HashMap`

Suppose:

```rust
let mut users = HashMap::new();

users.insert(101u64, "Aariv");
users.insert(102u64, "Wangdu");
users.insert(103u64, "Phunsuk");
```

You asked:

> Does `users` store pointer + length + capacity?

### Conceptually, yes, think of it as a small handle/metadata object.

But **don't memorize it as exactly `ptr + len + capacity`** like `Vec`.

A HashMap needs information about its hash table allocation and how much of it is occupied, and its exact representation is an implementation detail.

Conceptually:

```text
Stack

users
┌─────────────────────┐
│ hash-table metadata │
│ pointer/reference ──┼──────────────┐
│ size/length         │              │
│ capacity/table info │              │
└─────────────────────┘              │
                                     ↓
Heap

Hash table

┌─────────┬─────────────┐
│ bucket 0│ empty       │
├─────────┼─────────────┤
│ bucket 1│ 102 → ...   │
├─────────┼─────────────┤
│ bucket 2│ empty       │
├─────────┼─────────────┤
│ bucket 3│ 101 → ...   │
├─────────┼─────────────┤
│ ...     │ ...         │
└─────────┴─────────────┘
```

The heap allocation contains the **hash-table storage**, including the information needed to identify where entries are and the key/value data.

---

# But how does `HashMap` remember where the elements are?

This is the crucial part.

Suppose:

```text
key = 102
value = "Wangdu"
```

The HashMap computes a hash:

```text
102
 ↓
hash function
 ↓
hash value
 ↓
table position
```

That table position is associated with the entry.

So the HashMap doesn't need:

```text
"Here is a pointer to every single element in the collection."
```

Instead, it has a **table structure** that lets it determine where to look based on the key.

Conceptually:

```text
users.get(&102)

       102
        │
        ↓
    hash(102)
        │
        ↓
   table position
        │
        ↓
   ┌───────────────┐
   │ key = 102     │
   │ value = ...   │
   └───────────────┘
```

That's why it can avoid scanning all 3 elements.

---

# Compare all five

For **three `u64` values**, conceptually:

| Collection        | Where elements live        | Main idea                         |
| ----------------- | -------------------------- | --------------------------------- |
| `[u64; 3]`        | directly in the array      | 3 × `u64`                         |
| `Vec<u64>`        | contiguous heap allocation | pointer + len + capacity → data   |
| `VecDeque<u64>`   | contiguous heap allocation | buffer + logical front/back       |
| `LinkedList<u64>` | separate heap nodes        | nodes + prev/next pointers        |
| `HashMap<u64, V>` | hash-table allocation      | buckets/table + key/value entries |

### The most important mental picture

```text
ARRAY

[a][b][c]
 ↑
data itself


VEC

Vec descriptor ─────→ [a][b][c]
                       heap


VECDEQUE

Deque descriptor ────→ [a][b][c][ ][ ]
                         ↑
                    logical front


LINKEDLIST

List
 ├── head ──→ [a] ⇄ [b] ⇄ [c] ←── tail


HASHMAP

Map
 └── table ──→ [bucket][bucket][bucket][bucket]
                    ↓
                 key/value
```

And this is why their performance differs:

```text
Array
→ direct contiguous storage

Vec
→ direct indexing into contiguous storage

VecDeque
→ direct indexing + efficient two ends

LinkedList
→ pointer traversal between nodes

HashMap
→ hash key → locate table position
```

One final correction to keep your mental model clean: **a bucket isn't necessarily a little heap object containing a `key/value` pair and a pointer.** That's a useful teaching abstraction, but Rust's actual `HashMap` implementation uses a more sophisticated table layout. For learning algorithms, understand **“hash → table slot → locate entry”** first; exact byte layout comes later.
