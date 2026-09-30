# Level 1

For every data structure, understand:

Where is the data? stack / heap / static / etc.
Is memory contiguous or scattered?
How do I find an element?
Time complexity: O(1), O(log n), O(n)
Space complexity: O(n)
What happens when it grows?
Does it cause allocations/copies?


```text
Vec
→ contiguous heap memory
→ index O(1)
→ push back O(1) amortized

LinkedList
→ separate heap nodes
→ index O(n)
→ front/back O(1)

HashMap
→ heap memory
→ lookup O(1) average
→ extra capacity/control metadata

BTreeMap
→ heap-allocated tree nodes
→ sorted
→ lookup O(log n)

```

# Level 2 — Know the internals when they matter
allocation
alignment
padding
cache locality
pointer size
stack vs heap
ownership
borrowing
allocation/reallocation
memory fragmentation

# Level 3 — Exact memory analysis
Why are we using 8/some GB RAM?
Which structure is allocating?
How much memory does each connection consume?
Are we wasting 16 bytes per object?
Are allocations causing fragmentation?
Are cache misses killing performance?
