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
use itertools::Itertools;
use ordtools::Ordtools;
use proconio::input;

#[allow(clippy::many_single_char_names)]
fn enumerate(w: &[u64], v: &[u64]) -> Vec<(u64, u64)> {
    let n = w.len();
    if n == 0 {
        vec![(0, 0)]
    } else {
        let a = enumerate(&w[1..], &v[1..]);
        let mut ans = Vec::with_capacity(a.len() * 2);
        for (w, v) in a.iter().copied().merge_by(
            a.iter().map(|(wi, vi)| (w[0] + wi, v[0] + vi)),
            |&(w0, v0), &(w1, v1)| w0 < w1 || w0 == w1 && v0 >= v1,
        ) {
            if ans.last().map(|&(w0, v0)| w0 < w && v0 < v).unwrap_or(true) {
                ans.push((w, v));
            }
        }
        ans
    }
}

#[allow(clippy::many_single_char_names)]
fn main() {
    input!(n: usize, w: [u64; n], v: [u64; n], cap: u64);
    let a = enumerate(&w[..n / 2], &v[..n / 2]);
    let mut b = enumerate(&w[n / 2..], &v[n / 2..]);
    b.reverse();
    let mut j = 0;
    let mut ans = 0;
    for &(wa, va) in &a {
        while j < b.len() && cap < wa + b[j].0 {
            j += 1;
        }
        if cap < wa + b[j].0 {
            break;
        }
        ans.change_max(va + b[j].1);
    }
    println!("{}", ans);
}

#[cfg(test)]
mod chap3_2_huge_knapsack_tests {
    const BIN: &str = "chap3_2_huge_knapsack";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"4
2 1 3 2
3 2 4 2
5
"#,
            "7\n",
        );
    }
}
