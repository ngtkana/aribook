#![allow(clippy::many_single_char_names)]
use std::{fmt::Debug, ops::Range};

pub struct SparseTableArgmin<Value: Debug + Clone + Ord> {
    pub table: Vec<Vec<usize>>,
    pub values: Vec<Value>,
}

impl<Value: Debug + Clone + Ord> SparseTableArgmin<Value> {
    pub fn with_slice(slice: &[Value]) -> Self {
        Self::from_vec(slice.to_vec())
    }

    pub fn from_vec(vec: Vec<Value>) -> Self {
        let n = vec.len();
        let mut table = vec![(0..n).collect::<Vec<usize>>()];
        let mut d = 1;
        while d < n {
            let mut row = table.last().unwrap().clone();
            for i in 0..n - d {
                if vec[row[i]] > vec[row[i + d]] {
                    row[i] = row[i + d];
                }
            }
            table.push(row);
            d *= 2;
        }
        Self { table, values: vec }
    }

    pub fn argmin(&self, range: Range<usize>) -> usize {
        let Range { start, end } = range;
        assert!(start < end);
        if start + 1 == end {
            start
        } else {
            let d = (end - start).next_power_of_two() / 2;
            assert!(
                (start..=start + d).contains(&(end - d)) && (end - d..=end).contains(&(start + d))
            );
            let i = d.trailing_zeros() as usize;
            let x = self.table[i][start];
            let y = self.table[i][end - d];
            if self.values[x] > self.values[y] {
                y
            } else {
                x
            }
        }
    }
}

#[cfg(test)]
mod chap4_3_sparse_table_argmin {
    use super::*;

    #[test]
    fn test_sparse_table_argmin() {
        let a = vec![4, 5, 2, 1, 3];
        let spt = SparseTableArgmin::<u32>::with_slice(&a);
        assert_eq!(spt.argmin(0..5), 3);
        assert_eq!(spt.argmin(0..4), 3);
        assert_eq!(spt.argmin(3..4), 3);
        assert_eq!(spt.argmin(4..5), 4);
        assert_eq!(spt.argmin(2..3), 2);
        assert_eq!(spt.argmin(1..3), 2);
    }
}
