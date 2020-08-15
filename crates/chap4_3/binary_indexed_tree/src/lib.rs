#![allow(clippy::many_single_char_names)]

use std::{
    fmt::Debug,
    iter::successors,
    ops::{Add, Range, Sub},
};

// NOTE: どのトレイトを要求するか悩むところです。
// - Copy を要求する？
// - &Value: Add<Output = Value> など一式要求する？
// - いまのまま毎回 Clone？
pub struct BinaryIndexedTree<Value: Debug + Clone + Add<Output = Value>> {
    table: Vec<Value>,
}

impl<Value: Debug + Clone + Add<Output = Value>> BinaryIndexedTree<Value> {
    pub fn new() -> Self {
        Self { table: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            table: Vec::with_capacity(capacity),
        }
    }

    pub fn with_slice(slice: &[Value]) -> Self {
        let mut ans = Self::with_capacity(slice.len());
        for x in slice {
            ans.push(x.clone());
        }
        ans
    }

    pub fn push(&mut self, value: Value) {
        let value = (0..(self.table.len() + 1).trailing_zeros())
            .map(|j| self.table[self.table.len() - (1 << j)].clone())
            .fold(value, Add::add);
        self.table.push(value);
    }

    pub fn sum(&self, i: usize) -> Option<Value> {
        let mut map = successors(Some(i as i32), |&i| Some(i - (i & -i)))
            .take_while(|&i| i != 0)
            .map(|i: i32| self.table[i as usize - 1].clone());
        let first = map.next()?;
        Some(map.fold(first, Add::add))
    }
}

impl<Value: Debug + Clone + Add<Output = Value> + Sub<Output = Value>> BinaryIndexedTree<Value> {
    pub fn range_sum(&self, range: Range<usize>) -> Option<Value> {
        let Range { start, end } = range;
        if start >= end {
            None
        } else {
            let end_value = self.sum(end)?;
            Some(if let Some(start_value) = self.sum(start) {
                end_value - start_value
            } else {
                end_value
            })
        }
    }
}

#[cfg(test)]
mod chap4_3_binary_indexed_tree_tests {
    use super::*;

    #[test]
    fn test_push_sum() {
        let mut bit = BinaryIndexedTree::<i32>::new();
        bit.push(2);
        bit.push(3);
        bit.push(5);
        bit.push(7);
        assert_eq!(bit.sum(0), None);
        assert_eq!(bit.sum(1), Some(2));
        assert_eq!(bit.sum(2), Some(5));
        assert_eq!(bit.sum(3), Some(10));
        assert_eq!(bit.sum(4), Some(17));
    }

    #[test]
    fn test_with_slice_range_sum() {
        let bit = BinaryIndexedTree::<i32>::with_slice(&[2, 3, 5, 7]);
        assert_eq!(bit.range_sum(0..1), Some(2));
        assert_eq!(bit.range_sum(1..1), None);
        assert_eq!(bit.range_sum(2..1), None);
        assert_eq!(bit.range_sum(2..4), Some(12));
        assert_eq!(bit.range_sum(0..4), Some(17));
    }
}
