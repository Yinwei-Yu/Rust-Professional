/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;
use std::cmp::Ordering;

pub struct BinaryHeap<T> {
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T: Default + Clone + Ord> BinaryHeap<T> {
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
        self.items.push(value);
        self.count += 1;
        self.bubble_up(self.count);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        idx * 2 + 1
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn bubble_up(&mut self, mut idx: usize) {
        while idx > 1 {
            let parent = self.parent_idx(idx); // 预存父节点索引
            if (self.comparator)(&self.items[idx], &self.items[parent]) {
                self.items.swap(idx, parent); 
                idx = parent;
            } else {
                break;
            }
        }
    }

    fn bubble_down(&mut self, mut idx: usize) {
        while self.children_present(idx) {
            let mut child_idx = self.left_child_idx(idx);
            if child_idx < self.count && (self.comparator)(&self.items[child_idx + 1], &self.items[child_idx]) {
                child_idx += 1;
            }
            if (self.comparator)(&self.items[idx], &self.items[child_idx]) {
                break;
            }
            self.items.swap(idx, child_idx);
            idx = child_idx;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.items.swap(1, self.count);
        let result = self.items.pop();
        self.count -= 1;
        self.bubble_down(1);
        result
    }
}

fn main() {
    let mut heap = BinaryHeap::new(|a: &i32, b: &i32| a < b);
    heap.add(3);
    heap.add(1);
    heap.add(4);
    heap.add(1);
    heap.add(5);
    heap.add(9);

    while let Some(value) = heap.pop() {
        println!("{}", value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = BinaryHeap::new(|a: &i32, b: &i32| a > b);
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = BinaryHeap::new(|a: &i32, b: &i32| a < b);
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.pop(), Some(2));
        assert_eq!(heap.pop(), Some(4));
        assert_eq!(heap.pop(), Some(9));
        heap.add(1);
        assert_eq!(heap.pop(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = BinaryHeap::new(|a: &i32, b: &i32| a > b);
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.pop(), Some(11));
        assert_eq!(heap.pop(), Some(9));
        assert_eq!(heap.pop(), Some(4));
        heap.add(1);
        assert_eq!(heap.pop(), Some(2));
    }
}