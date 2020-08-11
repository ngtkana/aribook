// dbg {{{
#[allow(dead_code)]
mod dbg {
    use std::fmt::{Debug, Formatter};

    #[derive(Clone)]
    pub struct Tabular<'a, T: Debug>(pub &'a [T]);
    impl<'a, T: Debug> Debug for Tabular<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            for i in 0..self.0.len() {
                writeln!(f, "{:2} | {:?}", i, &self.0[i])?;
            }
            Ok(())
        }
    }

    #[derive(Clone)]
    pub struct BooleanTable<'a>(pub &'a [Vec<bool>]);
    impl<'a> Debug for BooleanTable<'a> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            for i in 0..self.0.len() {
                writeln!(f, "{:2} | {:?}", i, BooleanSlice(&self.0[i]))?;
            }
            Ok(())
        }
    }

    #[derive(Clone)]
    pub struct BooleanSlice<'a>(pub &'a [bool]);
    impl<'a> Debug for BooleanSlice<'a> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(
                f,
                "{}",
                self.0
                    .iter()
                    .map(|&b| if b { "1 " } else { "0 " })
                    .collect::<String>()
            )?;
            Ok(())
        }
    }
}
// }}}
use std::ops::{Bound, Range, RangeBounds};

pub struct TwoDimensionalBinaryIndexedTree {
    height: usize,
    width: usize,
    table: Vec<Vec<i32>>,
}

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

impl TwoDimensionalBinaryIndexedTree {
    pub fn with_height_width(height: usize, width: usize) -> Self {
        Self {
            table: vec![vec![0; width + 1]; height + 1],
            height,
            width,
        }
    }

    fn add_one_based(&mut self, mut i: usize, mut j: usize, x: i32) {
        let orig_j = j;
        while i <= self.height {
            while j <= self.width {
                self.table[i][j] += x;
                j += (j as i32 & -(j as i32)) as usize;
            }
            j = orig_j;
            i += (i as i32 & -(i as i32)) as usize;
        }
    }

    pub fn add(&mut self, i: usize, j: usize, x: i32) {
        self.add_one_based(i + 1, j + 1, x);
    }

    fn sum_up_one_based_closed(&self, mut i: usize, mut j: usize) -> i32 {
        let mut ans = 0;
        let orig_j = j;
        while i != 0 {
            while j != 0 {
                ans += self.table[i][j];
                j -= (j as i32 & -(j as i32)) as usize;
            }
            j = orig_j;
            i -= (i as i32 & -(i as i32)) as usize;
        }
        ans
    }

    pub fn sum_up(
        &self,
        i_range: impl RangeBounds<usize>,
        j_range: impl RangeBounds<usize>,
    ) -> i32 {
        let Range {
            start: i_start,
            end: i_end,
        } = range_bounds_to_range(i_range, self.height);
        let Range {
            start: j_start,
            end: j_end,
        } = range_bounds_to_range(j_range, self.width);
        self.sum_up_one_based_closed(i_end, j_end)
            - self.sum_up_one_based_closed(i_end, j_start)
            - self.sum_up_one_based_closed(i_start, j_end)
            + self.sum_up_one_based_closed(i_start, j_start)
    }
}

#[cfg(test)]
mod chap3_3_two_dimensional_binary_indexed_tree_tests {
    use super::*;

    #[test]
    fn test_two_dimensional_binary_indexed_tree() {
        let height = 10;
        let width = 20;
        let operation_count = 300;
        let random_value_breadth = 20;

        let mut table = vec![vec![0; width]; height];
        let mut bit = TwoDimensionalBinaryIndexedTree::with_height_width(height, width);
        for operation_enumerator in 0..operation_count {
            println!(
                "Started the {} th cycle of operation.",
                operation_enumerator
            );

            // add
            let i = rand::random::<usize>() % height;
            let j = rand::random::<usize>() % width;
            let x = rand::random::<i32>() % random_value_breadth;
            println!("Calling `add` by i = {}, j = {}, x = {}...", i, j, x);
            table[i][j] += x;
            bit.add(i, j, x);
            println!("Now the expected table is:\n{:?}", dbg::Tabular(&table));

            // sum_up
            let mut i0 = rand::random::<usize>() % height;
            let mut i1 = rand::random::<usize>() % height;
            let mut j0 = rand::random::<usize>() % width;
            let mut j1 = rand::random::<usize>() % width;
            if i0 > i1 {
                std::mem::swap(&mut i0, &mut i1);
            }
            if j0 > j1 {
                std::mem::swap(&mut j0, &mut j1);
            }
            i1 += 1;
            j1 += 1;
            let i_range = i0..i1;
            let j_range = j0..j1;
            println!(
                "Testing `sum_up` by i_range = {:?}, j_range = {:?}...",
                &i_range, &j_range
            );

            let expected = table[i_range.clone()]
                .iter()
                .map(|v| v[j_range.clone()].iter())
                .flatten()
                .sum::<i32>();
            assert_eq!(bit.sum_up(i_range, j_range), expected);
            println!();
        }
    }
}
