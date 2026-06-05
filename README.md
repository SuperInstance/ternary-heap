# ternary-heap

A **ternary min-heap** — each node has up to three children, giving O(log₃ n) `push` and `pop` with O(1) `peek`.

---

## What Problem Does This Solve?

A heap is the canonical data structure for **priority queues**: we repeatedly need the smallest (or largest) element, and we need to insert new elements efficiently.

### Binary vs. Ternary

In a binary heap each node has 2 children; in a ternary heap each node has **3**. Because the tree is shorter, the number of comparisons and swaps per `sift_up` / `sift_down` changes:

| Property | Binary Heap | **Ternary Heap** |
|----------|------------|-----------------|
| Children per node | 2 | 3 |
| Height | ⌊log₂ n⌋ | ⌊log₃ n⌋ |
| `push` swaps (worst) | log₂ n | log₃ n |
| `pop` comparisons per level | 1 | 2 (find min of 3) |

**Trade-off:** Each `sift_down` step in a ternary heap must compare **two siblings** to find the minimum child, whereas a binary heap compares only one. However, the path from root to leaf is roughly **37 % shorter** (log₃ n ≈ 0.63 · log₂ n). In practice, this can improve cache locality because the flat `Vec` representation traverses fewer indices.

### Array Indexing Formulas

The heap is stored as a flat `Vec`. For a node at index `i`:

- **Parent:** `(i - 1) / 3`
- **Children:** `3i + 1`, `3i + 2`, `3i + 3`

These formulas are the direct generalisation of the familiar binary-heap indices `2i + 1` and `2i + 2`.

---

## Mathematical Complexity Analysis

| Operation | Binary Heap | **Ternary Heap** | Proof Sketch |
|-----------|------------|-----------------|--------------|
| `peek`    | O(1)       | **O(1)**        | Root is at index 0. |
| `push`    | O(log₂ n)  | **O(log₃ n)**   | At most one swap per level; height = log₃ n. |
| `pop`     | O(log₂ n)  | **O(log₃ n)**   | Replace root with last element, sift down height levels. |
| `from_vec`| O(n)       | **O(n)**        | Floyd’s build-heap: sift down from last non-leaf backward. |
| `merge`   | O(m log n) | **O(m log₃ n)** | Repeated `push` of m elements. |

### Why `from_vec` Is O(n)

Floyd’s algorithm starts at the last non-leaf node and sifts each downward. In a ternary heap:
- At most ⌈n / 3⌉ nodes sit at height 1 and sift 1 level.
- At most ⌈n / 9⌉ nodes sit at height 2 and sift 2 levels.
- …and so on.

Total work is bounded by:

$$T(n) \leq \sum_{h=1}^{\log_3 n} \frac{n}{3^h} \cdot h = n \sum_{h=1}^{\infty} \frac{h}{3^h} = n \cdot \frac{3}{4} = O(n)$$

(Using the identity $\sum_{h=1}^{\infty} h x^h = \frac{x}{(1-x)^2}$ with $x = \frac{1}{3}$.)

---

## Architecture

### Logical Structure

```text
              1
        ┌─────┼─────┐
        3     5     7
      ┌─┼─┐ ┌─┼─┐ ┌─┼─┐
      9 11 13 15 17 19 21 23 25
```

### Flat Array Mapping

```text
Index:  0   1   2   3   4   5   6   7   8   9  ...
Value:  1   3   5   7   9  11  13  15  17  19  ...

Parent(4)  = (4 - 1) / 3 = 1  → value 3
Children(1) = 3·1+1=4, 3·1+2=5, 3·1+3=6 → values 9, 11, 13
```

### Heap Invariant

For every node `i` (except the root):

$$\text{data}[\text{parent}(i)] \leq \text{data}[i]$$

Equivalently, every parent is less than or equal to all of its up-to-three children.

---

## Getting Started

```rust
use ternary_heap::TernaryHeap;

fn main() {
    let mut heap = TernaryHeap::new();

    // Push some priorities
    heap.push(30);
    heap.push(10);
    heap.push(20);

    // Peek at the minimum without removing it
    assert_eq!(heap.peek(), Some(&10));

    // Pop in ascending order
    while let Some(v) = heap.pop() {
        println!("{}", v);
    }
    // Prints: 10, 20, 30

    // Build a heap from an existing vector in O(n)
    let data = vec![9, 3, 7, 1, 5, 8, 2];
    let heap = TernaryHeap::from_vec(data);
    assert_eq!(heap.drain_sorted(), vec![1, 2, 3, 5, 7, 8, 9]);
}
```

---

## Running the Tests

Run the full suite with:

```bash
cargo test
```

There are **12 tests**, each verifying a critical invariant:

| Test | What It Verifies |
|------|-----------------|
| `test_empty_heap` | An empty heap has length 0 and `peek()` returns `None`. |
| `test_push_peek` | After pushing 5, 1, 3, the minimum (1) is visible at the root. |
| `test_pop_min_order` | Seven elements are popped in strictly ascending order. |
| `test_pop_empty` | Popping an empty heap returns `None` without panic. |
| `test_single_element` | One push followed by one pop leaves the heap empty. |
| `test_duplicate_values` | Multiple equal keys are handled correctly and maintain total order. |
| `test_decrease_key` | Decreasing a key from 30 to 5 percolates it to the root. |
| `test_merge` | Merging two heaps concatenates their elements and preserves the heap invariant. |
| `test_drain_sorted` | `drain_sorted()` yields a fully sorted vector (heap sort). |
| `test_from_vec` | Building from a 9-element vector places the minimum at the root. |
| `test_large_heap` | 100 reverse-ordered elements are drained into perfect ascending order. |
| `test_three_children_invariant` | Every parent is ≤ all three of its children, checked exhaustively. |

---

## Related Crates

Explore the broader ternary ecosystem on crates.io:

- [`ternary-tree`](https://crates.io/crates/ternary-tree) — General-purpose ternary tree structures.
- [`ternary-compression`](https://crates.io/crates/ternary-compression) — Data compression using ternary alphabets.
- [`ternary-memory`](https://crates.io/crates/ternary-memory) — Ternary-addressable memory abstractions.
- [`ternary-tensor`](https://crates.io/crates/ternary-tensor) — Ternary-valued tensors for machine learning.

---

## License

MIT
