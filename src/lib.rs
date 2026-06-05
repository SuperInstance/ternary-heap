//! Ternary min-heap: each node has up to 3 children.
//! O(log₃ n) push/pop, O(1) peek.
//!
//! Stored as a flat Vec; for node at index i:
//!   children: 3i+1, 3i+2, 3i+3
//!   parent:   (i-1)/3

use std::cmp::Ordering;

/// Min-heap where each node has up to 3 children.
#[derive(Debug, Clone)]
pub struct TernaryHeap<T: Ord + Clone> {
    data: Vec<T>,
}

impl<T: Ord + Clone> Default for TernaryHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + Clone> TernaryHeap<T> {
    pub fn new() -> Self {
        TernaryHeap { data: Vec::new() }
    }

    pub fn with_capacity(cap: usize) -> Self {
        TernaryHeap { data: Vec::with_capacity(cap) }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
        let last = self.data.len() - 1;
        self.sift_up(last);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let last = self.data.len() - 1;
        self.data.swap(0, last);
        let min = self.data.pop();
        if !self.data.is_empty() {
            self.sift_down(0);
        }
        min
    }

    /// Decrease the key at `index` to `new_val` (must be ≤ current value).
    pub fn decrease_key(&mut self, index: usize, new_val: T) {
        if index >= self.data.len() {
            return;
        }
        if new_val <= self.data[index] {
            self.data[index] = new_val;
            self.sift_up(index);
        }
    }

    /// Merge another heap into this one (destructive of `other`).
    pub fn merge(&mut self, other: TernaryHeap<T>) {
        for item in other.data {
            self.push(item);
        }
    }

    fn parent(i: usize) -> Option<usize> {
        if i == 0 { None } else { Some((i - 1) / 3) }
    }

    fn first_child(i: usize) -> usize {
        3 * i + 1
    }

    fn sift_up(&mut self, mut i: usize) {
        while let Some(p) = Self::parent(i) {
            if self.data[i] < self.data[p] {
                self.data.swap(i, p);
                i = p;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut i: usize) {
        let n = self.data.len();
        loop {
            let fc = Self::first_child(i);
            if fc >= n {
                break;
            }
            // find minimum child among up to 3
            let mut min_child = fc;
            for c in (fc + 1)..(fc + 3).min(n) {
                if self.data[c] < self.data[min_child] {
                    min_child = c;
                }
            }
            if self.data[min_child] < self.data[i] {
                self.data.swap(i, min_child);
                i = min_child;
            } else {
                break;
            }
        }
    }

    /// Drain all elements in sorted order (heap sort).
    pub fn drain_sorted(mut self) -> Vec<T> {
        let mut out = Vec::with_capacity(self.data.len());
        while let Some(x) = self.pop() {
            out.push(x);
        }
        out
    }

    /// Build a heap from a vec in O(n).
    pub fn from_vec(v: Vec<T>) -> Self {
        let mut h = TernaryHeap { data: v };
        if h.data.len() > 1 {
            // start from last non-leaf
            let start = (h.data.len() - 2) / 3;
            for i in (0..=start).rev() {
                h.sift_down(i);
            }
        }
        h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let h: TernaryHeap<i32> = TernaryHeap::new();
        assert!(h.is_empty());
        assert_eq!(h.len(), 0);
        assert!(h.peek().is_none());
    }

    #[test]
    fn test_push_peek() {
        let mut h = TernaryHeap::new();
        h.push(5);
        h.push(1);
        h.push(3);
        assert_eq!(h.peek(), Some(&1));
        assert_eq!(h.len(), 3);
    }

    #[test]
    fn test_pop_min_order() {
        let mut h = TernaryHeap::new();
        for &x in &[9, 3, 7, 1, 5, 8, 2] {
            h.push(x);
        }
        let mut out = Vec::new();
        while let Some(v) = h.pop() {
            out.push(v);
        }
        assert_eq!(out, vec![1, 2, 3, 5, 7, 8, 9]);
    }

    #[test]
    fn test_pop_empty() {
        let mut h: TernaryHeap<i32> = TernaryHeap::new();
        assert!(h.pop().is_none());
    }

    #[test]
    fn test_single_element() {
        let mut h = TernaryHeap::new();
        h.push(42);
        assert_eq!(h.pop(), Some(42));
        assert!(h.is_empty());
    }

    #[test]
    fn test_duplicate_values() {
        let mut h = TernaryHeap::new();
        for _ in 0..5 {
            h.push(3);
        }
        h.push(1);
        h.push(5);
        assert_eq!(h.pop(), Some(1));
        for _ in 0..5 {
            assert_eq!(h.pop(), Some(3));
        }
        assert_eq!(h.pop(), Some(5));
    }

    #[test]
    fn test_decrease_key() {
        let mut h = TernaryHeap::new();
        h.push(10);
        h.push(20);
        h.push(30);
        h.decrease_key(2, 5); // decrease 30 to 5
        assert_eq!(h.peek(), Some(&5));
    }

    #[test]
    fn test_merge() {
        let mut h1 = TernaryHeap::new();
        h1.push(3);
        h1.push(7);
        let mut h2 = TernaryHeap::new();
        h2.push(1);
        h2.push(5);
        h1.merge(h2);
        assert_eq!(h1.len(), 4);
        assert_eq!(h1.pop(), Some(1));
        assert_eq!(h1.pop(), Some(3));
    }

    #[test]
    fn test_drain_sorted() {
        let mut h = TernaryHeap::new();
        for &x in &[5, 2, 8, 1, 9, 3] {
            h.push(x);
        }
        assert_eq!(h.drain_sorted(), vec![1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn test_from_vec() {
        let v = vec![9, 3, 7, 1, 5, 8, 2, 4, 6];
        let h = TernaryHeap::from_vec(v);
        assert_eq!(h.peek(), Some(&1));
        assert_eq!(h.len(), 9);
    }

    #[test]
    fn test_large_heap() {
        let mut h = TernaryHeap::new();
        for i in (0..100i32).rev() {
            h.push(i);
        }
        let sorted = h.drain_sorted();
        assert_eq!(sorted, (0..100i32).collect::<Vec<_>>());
    }

    #[test]
    fn test_three_children_invariant() {
        // After building, every parent should be ≤ all of its children
        let mut h = TernaryHeap::new();
        for &x in &[5, 1, 8, 3, 9, 2, 7, 4, 6] {
            h.push(x);
        }
        let n = h.len();
        for i in 0..n {
            let fc = 3 * i + 1;
            for c in fc..(fc + 3).min(n) {
                assert!(h.data[i] <= h.data[c], "heap violation at {i} vs {c}");
            }
        }
    }
}
