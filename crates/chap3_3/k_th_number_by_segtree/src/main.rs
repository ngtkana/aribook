// slicetools {{{
#[allow(dead_code)]
mod slicetools {
    pub trait Partitioned {
        type Item;

        fn partition_point(&self, f: impl Fn(&Self::Item) -> bool) -> usize;
    }

    impl<T> Partitioned for [T] {
        type Item = T;
        fn partition_point(&self, f: impl Fn(&Self::Item) -> bool) -> usize {
            if f(&self[0]) {
                0
            } else {
                let mut l = 0;
                let mut r = self.len();
                while 1 < r - l {
                    let c = l + (r - l) / 2;
                    match f(&self[c]) {
                        false => {
                            l = c;
                        }
                        true => {
                            r = c;
                        }
                    }
                }
                r
            }
        }
    }

    pub trait Sorted: Partitioned
    where
        Self::Item: Ord,
    {
        fn lower_bound(&self, x: &Self::Item) -> usize {
            self.partition_point(|y| x <= y)
        }

        fn upper_bound(&self, x: &Self::Item) -> usize {
            self.partition_point(|y| x < y)
        }
    }

    impl<T: Partitioned + ?Sized> Sorted for T where T::Item: Ord {}
}
// }}}
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
use proconio::{input, marker::Usize1};
use slicetools::Sorted;
use std::ops::Range;

struct Segtree {
    pub table: Vec<Vec<u32>>,
}

impl Segtree {
    pub fn len(&self) -> usize {
        self.table.len() / 2
    }

    pub fn build(a: &[u32]) -> Self {
        let n = a.len();
        let mut table = vec![vec![]; 2 * n];
        (0..n).for_each(|i| table[n + i] = vec![a[i]]);
        for i in (1..n).rev() {
            table[i] = itertools::merge(
                table[2 * i].iter().copied(),
                table[2 * i + 1].iter().copied(),
            )
            .collect();
        }

        Self { table }
    }

    pub fn count_le(&self, range: Range<usize>, value: u32) -> usize {
        let Range { mut start, mut end } = range;
        start += self.len();
        end += self.len();
        let mut ans = 0;
        while start < end {
            if start % 2 == 1 {
                ans += self.table[start].upper_bound(&value);
                start += 1;
            }
            if end % 2 == 1 {
                end -= 1;
                ans += self.table[end].upper_bound(&value);
            }
            start /= 2;
            end /= 2;
        }
        ans
    }
}

#[allow(clippy::many_single_char_names)]
fn main() {
    input!(n: usize, q: usize, a: [u32; n]);
    let a_max = a.iter().max().unwrap();
    let seg = Segtree::build(&a);
    for _ in 0..q {
        // center 以下が k 個以上あるような最小の center を計算します。
        input!(l: Usize1, r: usize, k: usize);
        let mut lower = 0;
        let mut upper = a_max + 1;
        while 1 < upper - lower {
            let center = lower + (upper - lower) / 2;
            if k <= seg.count_le(l..r, center) {
                upper = center;
            } else {
                lower = center;
            }
        }
        println!("{}", upper);
    }
}

#[cfg(test)]
mod chap3_3_k_th_number_by_segtree_tests {
    const BIN: &str = "chap3_3_k_th_number_by_segtree";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"7 3
1 5 2 6 3 7 4
2 5 3
4 4 1
1 7 3
"#,
            r#"5
6
3
"#,
        );
    }
}
