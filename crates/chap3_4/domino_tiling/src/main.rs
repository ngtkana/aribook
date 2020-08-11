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
use proconio::input;

fn main() {
    input!(
        h: usize,
        w: usize,
        modulus: u32,
        g: [proconio::marker::Chars; h],
    );
    let g = g
        .iter()
        .map(|v| v.iter().map(|&c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut dp = vec![0; 1 << w];
    dp[0] = 1;
    for i in 0..h {
        for j in 0..w {
            let mut swp = vec![0; 1 << w];
            for bs in 0..1 << w {
                let x = dp[bs];
                if x == 0 {
                    continue;
                }
                if g[i][j] || bs >> j & 1 == 1 {
                    swp[bs & !(1 << j)] += x;
                } else {
                    if j < w - 1 && !g[i][j + 1] && bs >> (j + 1) & 1 == 0 {
                        swp[bs | 1 << (j + 1)] += x;
                    }
                    if i < h - 1 && !g[i + 1][j] {
                        swp[bs | 1 << j] += x;
                    }
                }
            }
            std::mem::swap(&mut dp, &mut swp);
            dp.iter_mut().for_each(|x| *x %= modulus);
        }
    }
    println!("{}", dp[0]);
}

#[cfg(test)]
mod chap3_4_domino_tiling_tests {
    const BIN: &str = "chap3_4_domino_tiling";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3 3 2000003
...
.#.
...
"#,
            "2\n",
        );
    }
}
