```rust
fn main() {
    let x = 10;
    let numbers = vec![20, 30, 40];

    println!("{}", x);
}
```
```text
HIGH ADDRESS
┌──────────────────────────────┐
│            STACK             │
│                              │
│ main() stack frame           │
│                              │
│ x = 10                       │
│ numbers = Vec {              │
│    ptr ───────────────┐      │
│    len = 3            │      │
│    capacity = 3       │      │
│ }                     │      │
└───────────────────────│──────┘
                        │
                        │
                        ↓
┌──────────────────────────────┐
│            HEAP              │
│                              │
│ [20][30][40]                 │
│                              │
└──────────────────────────────┘

┌──────────────────────────────┐
│            BSS               │
│ uninitialized globals        │
└──────────────────────────────┘

┌──────────────────────────────┐
│            DATA              │
│ initialized mutable globals  │
└──────────────────────────────┘

┌──────────────────────────────┐
│           RODATA             │
│ read-only constants/strings  │
└──────────────────────────────┘

┌──────────────────────────────┐
│            TEXT              │
│ machine instructions         │
│ main(), functions, etc.      │
└──────────────────────────────┘
LOW ADDRESS
```

### Arrays
```rust
let numbers = [10, 20, 30];
```
```text
STACK

numbers
┌────┬────┬────┐
│ 10 │ 20 │ 30 │
└────┴────┴────┘
```
```text
No separate heap allocation is required for this local array.

The size is known at compile time:

[i32; 3]
```

```rust
Vec
let numbers = vec![10, 20, 30];
```
```text


Conceptually:

STACK
┌──────────────────┐
│ ptr ─────────────┼─────┐
│ len = 3          │     │
│ capacity = 3     │     │
└──────────────────┘     │
                         ↓
HEAP                 ┌────┬────┬────┐
                     │ 10 │ 20 │ 30 │
                     └────┴────┴────┘

```
```text
Array
→ data directly inside the array

Vec
→ small Vec descriptor + heap allocation
```
### VecDeque

Now:
```rust
let mut q = VecDeque::new();

q.push_back(10);
q.push_back(20);
q.push_back(30);
```
Conceptually:
```text
STACK

q
┌──────────────────┐
│ ptr ─────────────┼─────┐
│ len              │     │
│ capacity         │     │
│ front position   │     │
└──────────────────┘     │
                         ↓
HEAP

        physical storage

     ┌────┬────┬────┬────┬────┬────┐
     │ 10 │ 20 │ 30 │    │    │    │
     └────┴────┴────┴────┴────┴────┘
       ↑              ↑
     front           back


┌────┬────┬────┬────┬────┬────┐
│ 70 │    │ 30 │ 40 │ 50 │ 60 │
└────┴────┴────┴────┴────┴────┘
  ↑         ↑
back      front
```
### LinkedList
```rust
let mut list = LinkedList::new();

list.push_back(10);
list.push_back(20);
list.push_back(30);
```

```text
STACK

list
┌────────────────┐
│ head ──────────┼────────────┐
│ tail ──────────┼───────┐    │
└────────────────┘       │    │
                         │    │
                         ↓    ↓
HEAP

       ┌─────────────┐
       │ 10          │
       │ next ───────┼──────────┐
       └─────────────┘          │
                                ↓
                         ┌─────────────┐
                         │ 20          │
                         │ next ───────┼──────┐
                         └─────────────┘      │
                                              ↓
                                       ┌─────────────┐
                                       │ 30          │
                                       │ next = null │
                                       └─────────────┘
```
