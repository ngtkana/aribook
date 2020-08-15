#![allow(clippy::many_single_char_names)]
// dbg {{{
#[allow(dead_code)]
mod dbg {
    #[macro_export]
    macro_rules! lg {
        () => {
            $crate::eprintln!("[{}:{}]", $crate::file!(), $crate::line!());
        };
        ($val:expr) => {
            match $val {
                tmp => {
                    eprintln!("[{}:{}] {} = {:?}",
                        file!(), line!(), stringify!($val), &tmp);
                    tmp
                }
            }
        };
        ($val:expr,) => { lg!($val) };
        ($($val:expr),+ $(,)?) => {
            ($(lg!($val)),+,)
        };
    }

    #[macro_export]
    macro_rules! msg {
            () => {
                compile_error!();
            };
            ($msg:expr) => {
                $crate::eprintln!("[{}:{}][{}]", $crate::file!(), $crate::line!(), $msg);
            };
            ($msg:expr, $val:expr) => {
                match $val {
                    tmp => {
                        eprintln!("[{}:{}][{}] {} = {:?}",
                            file!(), line!(), $msg, stringify!($val), &tmp);
                        tmp
                    }
                }
            };
            ($msg:expr, $val:expr,) => { msg!($msg, $val) };
            ($msg:expr, $($val:expr),+ $(,)?) => {
                ($(msg!($msg, $val)),+,)
            };
        }

    #[macro_export]
    macro_rules! tabular {
        ($val:expr) => {
            eprintln!(
                "[{}:{}] {}:\n{:?}",
                file!(),
                line!(),
                stringify!($val),
                crate::dbg::Tabular($val)
            );
        };
    }

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

use std::{
    fmt::Debug,
    iter::successors,
    ops::{Add, Range, Sub},
};

// NOTE: どのトレイトを要求するか悩むところです。
// - Copy を要求する？
// - &Value: Add<Output = Value> など一式要求する？
// - いまのまま毎回 Clone？
#[derive(Debug, Clone)]
pub struct BinaryIndexedTree<Value: Debug + Clone + Add<Output = Value>> {
    table: Vec<Value>,
}

impl<Value: Debug + Clone + Add<Output = Value>> Default for BinaryIndexedTree<Value> {
    fn default() -> Self {
        Self::new()
    }
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
        assert!(i <= self.table.len());
        let mut map = successors(Some(i as i32), |&i| Some(i - (i & -i)))
            .take_while(|&i| i != 0)
            .map(|i: i32| self.table[i as usize - 1].clone());
        let first = map.next()?;
        Some(map.fold(first, Add::add))
    }

    pub fn add(&mut self, i: usize, x: Value) {
        assert!(i < self.table.len());
        let len = self.table.len();
        for i in successors(Some(i as i32 + 1), |&i| Some(i + (i & -i)))
            .map(|i| (i - 1) as usize)
            .take_while(|&i| i < len)
        {
            self.table[i] = self.table[i].clone() + x.clone();
        }
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

    pub fn get(&self, i: usize) -> Option<Value> {
        if i < self.table.len() {
            Some(if i == 0 {
                self.sum(1).unwrap()
            } else {
                self.sum(i + 1).unwrap() - self.sum(i).unwrap()
            })
        } else {
            None
        }
    }

    pub fn update(&mut self, i: usize, x: Value) {
        assert!(i < self.table.len());
        let diff = x - self.get(i).unwrap();
        self.add(i, diff);
    }

    pub fn collect_vec(&self) -> Vec<Value> {
        (0..self.table.len())
            .map(|i| self.get(i).unwrap())
            .collect()
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
