use std::ops::{Bound, Range, RangeBounds};

fn range_bounds_to_range(range: impl RangeBounds<usize>, len: usize) -> Range<usize> {
    (match range.start_bound() {
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => x + 1,
        Bound::Unbounded => 0,
    })..(match range.end_bound() {
        Bound::Included(&x) => x + 1,
        Bound::Excluded(&x) => x,
        Bound::Unbounded => len,
    })
}

pub struct BinaryIndexedTree(Vec<i32>);

impl BinaryIndexedTree {
    pub fn with_len(len: usize) -> Self {
        Self(vec![0; len + 1])
    }

    fn sum_one_based_closed(&self, mut i: usize) -> i32 {
        let mut ans = 0;
        while i != 0 {
            ans += self.0[i as usize];
            i -= (i as i32 & -(i as i32)) as usize;
        }
        ans
    }

    pub fn sum(&self, range: impl RangeBounds<usize>) -> i32 {
        let Range { start, end } = range_bounds_to_range(range, self.0.len() - 1);
        self.sum_one_based_closed(end) - self.sum_one_based_closed(start)
    }

    pub fn add(&mut self, mut i: usize, x: i32) {
        i += 1;
        while i < self.0.len() {
            self.0[i as usize] += x;
            i += (i as i32 & -(i as i32)) as usize;
        }
    }
}

#[cfg(test)]
mod chap3_3_binary_indexed_tree_tests {
    use super::*;

    #[test]
    fn test_hand() {
        let mut bit = BinaryIndexedTree::with_len(5);
        bit.add(2, 4);
        bit.add(1, 3);
        assert_eq!(bit.sum(..0), 0);
        assert_eq!(bit.sum(..1), 0);
        assert_eq!(bit.sum(..2), 3);
        assert_eq!(bit.sum(..3), 7);
        assert_eq!(bit.sum(..4), 7);

        bit.add(3, -7);
        assert_eq!(bit.sum(..0), 0);
        assert_eq!(bit.sum(..1), 0);
        assert_eq!(bit.sum(..2), 3);
        assert_eq!(bit.sum(..3), 7);
        assert_eq!(bit.sum(..4), 0);
    }
}
