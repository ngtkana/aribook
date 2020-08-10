use proconio::input;
// dbg {{{
#[allow(dead_code)]
mod dbg {
    use std::fmt::{Debug, Formatter};

    #[derive(Clone)]
    pub struct Tabular<'a, T: Debug>(pub &'a [Vec<T>]);
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

fn main() {
    input!(n: usize, wv: [(usize, u32); n], cap: usize);
    let mut dp = vec![vec![0; cap as usize + 1]; n + 1];
    for (i, &(w, v)) in wv.iter().enumerate() {
        for j in 0..=cap {
            dp[i + 1][j] = if j < w {
                dp[i][j]
            } else {
                dp[i][j].max(v + dp[i][j - w])
            }
        }
    }
    println!("{}", dp[n][cap]);
}

#[cfg(test)]
mod samples {
    const BIN: &str = "chap2_3_01knapsack_another_dp";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"4
2 3
1 2
3 4
2 2
5
"#,
            "7\n",
        );
    }
}
