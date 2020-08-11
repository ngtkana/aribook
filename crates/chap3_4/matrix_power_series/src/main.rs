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

fn pow(mut a: Vec<Vec<u32>>, mut b: u64, modulus: u32) -> Vec<Vec<u32>> {
    let n = a.len();
    let mut ans = vec![vec![0; n]; n];
    (0..n).for_each(|i| ans[i][i] = 1);
    while 0 != b {
        if b % 2 == 1 {
            ans = mul(&ans, &a, modulus);
        }
        a = mul(&a, &a, modulus);
        b /= 2;
    }
    ans
}
fn mul(a: &[Vec<u32>], b: &[Vec<u32>], modulus: u32) -> Vec<Vec<u32>> {
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
        .for_each(|x| *x %= modulus);
    c
}

fn main() {
    input!(n: usize, k: u64, modulus: u32, mut a: [[u32; n]; n]);
    let orig_a = a.clone();
    a.append(&mut vec![vec![0; n]; n]);
    a.iter_mut().for_each(|v| v.append(&mut vec![0; n]));
    for i in 0..n {
        a[i][n + i] = 1;
        a[n + i][n + i] = 1;
    }
    let a_to_k = pow(a, k, modulus);
    let sum = mul(
        a_to_k[..n]
            .iter()
            .map(|v| v[n..].iter().copied().collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .as_slice(),
        &orig_a,
        modulus,
    );
    for v in sum {
        println!(
            "{}",
            v.iter()
                .map(|x| x.to_string())
                .fold(String::new(), |acc, x| if acc.is_empty() {
                    x
                } else {
                    acc + " " + x.as_ref()
                })
        );
    }
}

#[cfg(test)]
mod chap3_4_matrix_power_series_tests {
    const BIN: &str = "chap3_4_matrix_power_series";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"2 2 4
0 1
1 1
"#,
            "1 2\n2 3\n",
        );
    }
}
