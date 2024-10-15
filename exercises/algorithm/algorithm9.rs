use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default + Clone + Ord,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Clone + Ord,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.count += 1;
        self.items.push(value);
        let mut current_idx = self.count;
        while current_idx > 1 && (self.comparator)(&self.items[current_idx], &self.items[self.parent_idx(current_idx)]) {
            let parent_idx =self.parent_idx(current_idx);
            self.items.swap(current_idx, parent_idx);
            current_idx = self.parent_idx(current_idx);
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        if!self.children_present(idx) {
            return idx;
        }
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);
        if right_idx > self.count {
            return left_idx;
        }
        if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
            return left_idx;
        } else {
            return right_idx;
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Clone,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Ord + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let min = self.items[1].clone();
        self.items[1] = self.items[self.count].clone();
        self.count -= 1;
        self.items.pop();
        let mut current_idx = 1;
        while self.children_present(current_idx) {
            let smallest_child_idx = self.smallest_child_idx(current_idx);
            if (self.comparator)(&self.items[smallest_child_idx], &self.items[current_idx]) {
                self.items.swap(current_idx, smallest_child_idx);
                current_idx = smallest_child_idx;
            } else {
                break;
            }
        }
        Some(min)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
    {
        Heap::new_min()
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
    {
        Heap::new_max()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}