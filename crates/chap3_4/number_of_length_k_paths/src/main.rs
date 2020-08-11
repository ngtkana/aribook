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

const MODULUS: u32 = 10_007;

#[allow(clippy::many_single_char_names)]
fn main() {
    fn pow(mut a: Vec<Vec<u32>>, mut b: u64) -> Vec<Vec<u32>> {
        let n = a.len();
        let mut ans = vec![vec![0; n]; n];
        (0..n).for_each(|i| ans[i][i] = 1);
        while 0 != b {
            if b % 2 == 1 {
                ans = mul(&ans, &a);
            }
            a = mul(&a, &a);
            b /= 2;
        }
        ans
    }
    fn mul(a: &[Vec<u32>], b: &[Vec<u32>]) -> Vec<Vec<u32>> {
        let n = a.len();
        let mut c = vec![vec![0; n]; n];
        for (i, ai) in a.iter().enumerate() {
            for j in 0..n {
                for (k, bk) in b.iter().enumerate() {
                    c[i][j] += ai[k] * bk[j];
                }
            }
        }
        c.iter_mut()
            .map(|v| v.iter_mut())
            .flatten()
            .for_each(|x| *x %= MODULUS);
        c
    }

    input!(n: usize, m: usize, k: u64, uv: [(Usize1, Usize1); m]);
    let mut a = vec![vec![0; n]; n];
    for (u, v) in uv {
        a[u][v] = 1;
    }
    let b = pow(a, k);
    println!(
        "{}",
        b.iter().map(|v| v.iter()).flatten().sum::<u32>() % MODULUS
    );
}

#[cfg(test)]
mod chap3_4_number_of_length_k_paths_tests {
    const BIN: &str = "chap3_4_number_of_length_k_paths";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"4 5 2
1 2
1 3
2 3
3 4
4 1
"#,
            "6\n",
        );
    }
}
