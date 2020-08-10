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
                    .map(|&b| if b { '#' } else { '.' })
                    .collect::<String>()
            )?;
            Ok(())
        }
    }
}
// }}}
use proconio::input;

fn main() {
    input!(total: u64, mut n: usize, a: [u64; n]);
    let mut a = a
        .iter()
        .copied()
        .chain(std::iter::once(total + 1))
        .collect::<Vec<_>>();
    n += 1;
    for i in (1..n).rev() {
        a[i] -= a[i - 1];
    }

    let mut dp = vec![vec![std::u64::MAX; n + 1]; n + 1];
    (0..n).for_each(|i| dp[i][i + 1] = 0);
    for d in 2..=n {
        for l in 0..=n - d {
            let r = l + d;
            dp[l][r] = a[l..r].iter().sum::<u64>() - 2
                + (l + 1..r).map(|c| dp[l][c] + dp[c][r]).min().unwrap();
        }
    }
    println!("{}", dp[0][n]);
}

#[cfg(test)]
mod chap2_7_bridbe_the_prisoners_tests {
    const BIN: &str = "chap2_7_bridbe_the_prisoners";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"8 1
3
"#,
            "7\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"20 3
3 6 14
"#,
            "35\n",
        );
    }
}
