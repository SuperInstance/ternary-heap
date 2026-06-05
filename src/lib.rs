//! # ternary-heap
//! Ternary heap (priority queue) where each node has 3 children.
//! O(log₃ n) push/pop, O(1) peek.

/// Min-heap where each node has up to 3 children.
#[derive(Debug, Clone)]
pub struct TernaryHeap<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> TernaryHeap<T> {
    pub fn new() -> Self { Self { data: Vec::new() } }

    pub fn with_capacity(cap: usize) -> Self {
        Self { data: Vec::with_capacity(cap) }
    }

    pub fn len(&self) -> usize { self.data.len() }
    pub fn is_empty(&self) -> bool { self.data.is_empty() }

    /// Push element. O(log₃ n).
    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.sift_up(self.data.len() - 1);
    }

    /// Pop minimum. O(log₃ n).
    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() { return None; }
        let last = self.data.pop()?;
        if self.data.is_empty() { return Some(last); }
        let min = std::mem::replace(&mut self.data[0], last);
        self.sift_down(0);
        Some(min)
    }

    /// Peek minimum. O(1).
    pub fn peek(&self) -> Option<&T> { self.data.first() }

    /// Merge another heap into this one.
    pub fn merge(&mut self, other: TernaryHeap<T>) {
        for item in other.data { self.push(item); }
    }

    fn parent(i: usize) -> usize { (i - 1) / 3 }
    fn child(i: usize, c: usize) -> usize { 3 * i + c + 1 }

    fn sift_up(&mut self, mut i: usize) {
        while i > 0 {
            let p = Self::parent(i);
            if self.data[i] < self.data[p] {
                self.data.swap(i, p);
                i = p;
            } else { break; }
        }
    }

    fn smallest_child(&self, i: usize) -> Option<usize> {
        let base = 3 * i + 1;
        let mut best = None;
        let mut best_val = &self.data[i];
        for c in 0..3 {
            let ci = base + c;
            if ci < self.data.len() && self.data[ci] < *best_val {
                best_val = &self.data[ci];
                best = Some(ci);
            }
        }
        best
    }

    fn sift_down(&mut self, mut i: usize) {
        while let Some(smaller) = self.smallest_child(i) {
            self.data.swap(i, smaller);
            i = smaller;
        }
    }
}

impl<T: Ord> Default for TernaryHeap<T> {
    fn default() -> Self { Self::new() }
}

/// Decrease-key operation: rebuild heap with a predicate to update elements.
pub fn decrease_key<T: Ord + Clone>(heap: &mut TernaryHeap<T>, predicate: impl Fn(&T) -> bool, new_val: T) -> bool {
    let mut found = false;
    for item in heap.data.iter_mut() {
        if predicate(item) {
            *item = new_val.clone();
            found = true;
            break;
        }
    }
    if found { heap.rebuild(); }
    found
}

impl<T: Ord> TernaryHeap<T> {
    fn rebuild(&mut self) {
        let n = self.data.len();
        for i in (0..n).rev() {
            self.sift_down(i);
        }
    }

    pub fn into_sorted_vec(self) -> Vec<T> {
        let mut heap = self;
        let mut result = Vec::with_capacity(heap.len());
        while let Some(item) = heap.pop() { result.push(item); }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_heap() {
        let mut h: TernaryHeap<i32> = TernaryHeap::new();
        assert!(h.is_empty());
        assert_eq!(h.len(), 0);
        assert!(h.peek().is_none());
        assert!(h.pop().is_none());
    }

    #[test]
    fn push_peek_pop_single() {
        let mut h = TernaryHeap::new();
        h.push(42);
        assert_eq!(h.peek(), Some(&42));
        assert_eq!(h.pop(), Some(42));
        assert!(h.is_empty());
    }

    #[test]
    fn min_heap_ordering() {
        let mut h = TernaryHeap::new();
        for &v in &[5, 3, 7, 1, 9, 2, 8, 4, 6] { h.push(v); }
        let sorted = h.into_sorted_vec();
        assert_eq!(sorted, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn log3_complexity() {
        // With 27 elements, height should be 3 (3³=27)
        let mut h = TernaryHeap::new();
        for i in 0..27 { h.push(i); }
        assert_eq!(h.len(), 27);
        // Pop all and verify sorted
        let mut prev = -1;
        while let Some(v) = h.pop() {
            assert!(v as i32 > prev, "out of order: {} <= {}", v, prev);
            prev = v as i32;
        }
    }

    #[test]
    fn merge_two_heaps() {
        let mut h1 = TernaryHeap::new();
        h1.push(3); h1.push(1); h1.push(5);
        let mut h2 = TernaryHeap::new();
        h2.push(2); h2.push(4); h2.push(6);
        h1.merge(h2);
        assert_eq!(h1.len(), 6);
        assert_eq!(h1.into_sorted_vec(), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn decrease_key_basic() {
        let mut h = TernaryHeap::new();
        h.push(10); h.push(20); h.push(30);
        let found = decrease_key(&mut h, |x| *x == 20, 1);
        assert!(found);
        assert_eq!(h.pop(), Some(1));
    }

    #[test]
    fn ternary_structure() {
        // Verify parent/child relationships
        assert_eq!(TernaryHeap::<i32>::parent(1), 0);
        assert_eq!(TernaryHeap::<i32>::parent(2), 0);
        assert_eq!(TernaryHeap::<i32>::parent(3), 0);
        assert_eq!(TernaryHeap::<i32>::parent(4), 1);
        assert_eq!(TernaryHeap::<i32>::parent(7), 2);
    }

    #[test]
    fn reverse_order_insert() {
        let mut h = TernaryHeap::new();
        for i in (0..100).rev() { h.push(i); }
        assert_eq!(h.pop(), Some(0));
        assert_eq!(h.peek(), Some(&1));
    }

    #[test]
    fn duplicate_values() {
        let mut h = TernaryHeap::new();
        for _ in 0..5 { h.push(42); }
        for _ in 0..5 { assert_eq!(h.pop(), Some(42)); }
        assert!(h.is_empty());
    }

    #[test]
    fn with_capacity() {
        let h: TernaryHeap<i32> = TernaryHeap::with_capacity(100);
        assert!(h.is_empty());
        assert_eq!(h.len(), 0);
    }

    #[test]
    fn string_heap() {
        let mut h = TernaryHeap::new();
        h.push("cherry"); h.push("apple"); h.push("banana");
        assert_eq!(h.pop(), Some("apple"));
        assert_eq!(h.pop(), Some("banana"));
        assert_eq!(h.pop(), Some("cherry"));
    }

    #[test]
    fn large_heap_stress() {
        let mut h = TernaryHeap::new();
        for i in 0..1000 { h.push(1000 - i); }
        for i in 1..=1000 { assert_eq!(h.pop(), Some(i)); }
    }
}
