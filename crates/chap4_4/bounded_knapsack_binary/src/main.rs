#![allow(clippy::many_single_char_names)]
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
    input!(n: usize, wvm: [(usize, u32, usize); n], cap: usize);
    let mut dp = vec![0; cap + 1];
    for (w, v, mut m) in wvm {
        for i in 0.. {
            let m_now = (1 << i).min(m);
            for i in (w * m_now..=cap).rev() {
                let x = dp[i - w * m_now] + v * m_now as u32;
                dp[i].change_max(x);
            }
            m -= m_now;
            if m == 0 {
                break;
            }
        }
    }
    println!("{}", dp.last().unwrap());
}

#[cfg(test)]
mod chap4_4_bounded_knapsack_binary_tests {
    const BIN: &str = "chap4_4_bounded_knapsack_binary";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3
3 2 5
2 4 1
4 3 3
12
"#,
            "11\n",
        );
    }
}
