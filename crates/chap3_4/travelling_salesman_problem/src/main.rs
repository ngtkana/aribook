// ordtools {{{
#[allow(dead_code)]
mod ordtools {
    pub trait Ordtools: PartialOrd + Sized {
        fn change_min(&mut self, mut rhs: Self) {
            if self > &mut rhs {
                *self = rhs;
            }
        }

        fn change_max(&mut self, mut rhs: Self) {
            if self < &mut rhs {
                *self = rhs;
            }
        }
    }

    impl<T: PartialOrd + Sized> Ordtools for T {}
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
use ordtools::Ordtools;
use proconio::input;

fn main() {
    input!(n: usize, m: usize, uvw: [(usize, usize, u32); m]);
    let mut g = vec![vec![std::u32::MAX; n]; n];
    (0..n).for_each(|i| g[i][i] = 0);
    for (u, v, w) in uvw {
        g[u][v] = w;
    }

    let mut dp = vec![vec![std::u32::MAX; n]; 1 << n];
    dp[(1 << n) - 1][0] = 0;
    for bs in (0..(1 << n) - 1).rev() {
        for (i, gi) in g.iter().enumerate() {
            for j in (0..n).filter(|&j| bs >> j & 1 == 0) {
                let x = dp[bs | 1 << j][j].saturating_add(gi[j]);
                dp[bs][i].change_min(x);
            }
        }
    }
    println!("{}", dp[0][0]);
}

#[cfg(test)]
mod chap3_4_travelling_salesman_problem_tests {
    const BIN: &str = "chap3_4_travelling_salesman_problem";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"5 8
0 1 3
0 3 4
1 2 5
2 0 4
2 3 5
3 4 3
4 1 6
4 0 7
"#,
            "22\n",
        );
    }
}
