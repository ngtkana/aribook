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
use ordtools::Ordtools;
use proconio::input;

fn main() {
    input!(n: usize, k: u32, a: [u32; n]);
    let mut ans = n + 1;
    let mut r = 0;
    let mut sum = 0;
    for l in 0..n {
        while r < n && sum < k {
            sum += a[r];
            r += 1;
        }
        if sum < k {
            break;
        }
        ans.change_min(r - l);
        sum -= a[l];
    }
    if ans == n + 1 {
        ans = 0;
    }
    println!("{}", ans);
}

#[cfg(test)]
mod chap3_2_subsequence_tests {
    const BIN: &str = "chap3_2_subsequence";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"10 15
5 1 3 5 10 7 4 9 2 8
"#,
            "2\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"5 11
1 2 3 4 5
"#,
            "3\n",
        );
    }
}
