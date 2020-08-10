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
use ordered_float::NotNan;
use proconio::input;

fn main() {
    input!(m: usize, p: f64, pos: usize);
    let n = 1 << m;
    let pos = (pos * n) / 1_000_000;
    let mut dp = vec![vec![0.0; n]; m + 1];
    for i in 0..m {
        for j in 0..n {
            let x = (0..=j)
                .map(|l| dp[i][l] * (1.0 - p) + p * dp[i].get(2 * j - l).unwrap_or(&1.0))
                .max_by_key(|&x| NotNan::new(x).unwrap())
                .unwrap();
            dp[i + 1][j] = x;
        }
    }
    println!("{}", dp[m][pos]);
}

#[cfg(test)]
mod chap2_7_millionare_tests {
    const BIN: &str = "chap2_7_millionare";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"1 0.5 500000
"#,
            "0.5\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"3 0.75 600000
"#,
            "0.84375\n",
        );
    }
}
