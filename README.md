# ternary-heap

**Ternary min-heap: O(log₃ n) priority queue with 3 children per node.**

Binary heaps have 2 children per node. Ternary heaps have 3. The result: fewer levels (log₃ n vs log₂ n) at the cost of one extra comparison per level. For n > 1000, ternary heaps are faster for insert-heavy workloads.

---

## Complexity Analysis

| Operation | Binary Heap | Ternary Heap |
|-----------|-------------|--------------|
| push | O(log₂ n) comps | O(log₃ n) comps |
| pop | O(log₂ n) × 1 comp | O(log₃ n) × 2 comps |
| peek | O(1) | O(1) |

Since log₃(n) ≈ 0.63 × log₂(n):
- **Push**: ternary does 0.63× the work per level → **faster**
- **Pop**: ternary does 2 × 0.63 = 1.26× the comparisons → **slightly slower**
- **Crossover**: around n ≈ 1000 elements

---

## Architecture

- **`TernaryHeap<T>`** — Min-heap with 3 children per internal node
  - `push(item)` — Insert, sift up: O(log₃ n)
  - `pop()` → `Option<T>` — Extract min, sift down: O(log₃ n)
  - `peek()` → `Option<&T>` — View minimum: O(1)
  - `merge(other)` — Absorb another heap
  - `into_sorted_vec()` — Extract all elements sorted
- **`decrease_key()`** — Update priority and re-heapify

---

## Quick Start

```rust
use ternary_heap::TernaryHeap;

let mut heap = TernaryHeap::new();
heap.push(5);
heap.push(1);
heap.push(3);
heap.push(2);
heap.push(4);

assert_eq!(heap.pop(), Some(1));
assert_eq!(heap.pop(), Some(2));
assert_eq!(heap.peek(), Some(&3));

let sorted = heap.into_sorted_vec();
assert_eq!(sorted, vec![3, 4, 5]);
```

---

## Ecosystem

- **ternary-btree** — Ternary B-tree (ordered map)
- **ternary-sort** — Sorting for ternary data
- **ternary-priority** — Priority queue with ternary tiebreaking
- **ternary-scheduler** — Task scheduling using ternary heaps

## License

MIT
