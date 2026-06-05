# ternary-heap

Ternary min-heap (priority queue) where each node has 3 children.

O(log₃ n) push/pop, O(1) peek. Stored as a flat Vec.

## Features
- `TernaryHeap<T>` — generic min-heap
- `push`, `pop`, `peek`
- `decrease_key(index, new_val)`
- `merge(other)` — destructive merge
- `from_vec` — O(n) heapify
- `drain_sorted` — heap sort
- 12 tests

## Usage
```rust
let mut h = TernaryHeap::new();
h.push(5); h.push(1); h.push(3);
assert_eq!(h.pop(), Some(1));
```
