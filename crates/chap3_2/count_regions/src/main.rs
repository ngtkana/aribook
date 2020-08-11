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
use proconio::input;
use slicetools::Sorted;

fn main() {
    input!(
        h: u32,
        w: u32,
        n: usize,
        mut a0: [u32; n],
        a1: [u32; n],
        mut b0: [u32; n],
        b1: [u32; n]
    );
    a0.iter_mut().for_each(|z| *z -= 1);
    b0.iter_mut().for_each(|z| *z -= 1);
    let make_coord = |v0: &[u32], v1: &[u32], len: u32| {
        let mut v = v0
            .iter()
            .chain(v1.iter())
            .copied()
            .chain(std::iter::once(0))
            .chain(std::iter::once(len))
            .collect::<Vec<_>>();
        v.sort();
        v.dedup();
        v
    };
    let a_coords = make_coord(&a0, &a1, h);
    let b_coords = make_coord(&b0, &b1, w);

    let h = a_coords.len() - 1;
    let w = b_coords.len() - 1;

    let mut grid = vec![vec![false; w]; h];
    for i in 0..n {
        let x0 = a_coords.lower_bound(&a0[i]);
        let x1 = a_coords.lower_bound(&a1[i]);
        let y0 = b_coords.lower_bound(&b0[i]);
        let y1 = b_coords.lower_bound(&b1[i]);
        for v in grid.iter_mut().take(x1).skip(x0) {
            for z in v.iter_mut().take(y1).skip(y0) {
                *z = true;
            }
        }
    }

    let mut ans = 0;
    let mut queue = std::collections::VecDeque::new();
    for si in 0..h {
        for sj in 0..w {
            if grid[si][sj] {
                continue;
            }
            ans += 1;
            queue.push_back((si, sj));
            while let Some((i, j)) = queue.pop_front() {
                for (ni, nj) in [(1, 0), (-1, 0), (0, 1), (0, -1)]
                    .iter()
                    .filter_map(|&(di, dj)| {
                        let ni = (i as i32) + di;
                        let nj = (j as i32) + dj;
                        if 0 <= ni && ni < h as i32 && 0 <= nj && nj < w as i32 {
                            let ni = ni as usize;
                            let nj = nj as usize;
                            if std::mem::replace(&mut grid[ni][nj], true) {
                                None
                            } else {
                                Some((ni, nj))
                            }
                        } else {
                            None
                        }
                    })
                {
                    queue.push_back((ni, nj));
                }
            }
        }
    }
    println!("{}", ans)
}

#[cfg(test)]
mod chap3_2_count_regions_tests {
    const BIN: &str = "chap3_2_count_regions";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"10 10 5
1 1 4 9 10
6 10 4 9 10
4 8 1 1 6
4 8 10 5 10
"#,
            "6\n",
        );
    }
}
