use proconio::input;
// slicetools {{{
#[allow(dead_code)]
mod slicetools {
    pub trait Partitioned {
        type Item;

        fn partition_point(&self, f: impl Fn(&Self::Item) -> bool) -> usize;
    }

    impl<T> Partitioned for [T] {
        type Item = T;
        fn partition_point(&self, f: impl Fn(&Self::Item) -> bool) -> usize {
            if f(&self[0]) {
                0
            } else {
                let mut l = 0;
                let mut r = self.len();
                while 1 < r - l {
                    let c = l + (r - l) / 2;
                    match f(&self[c]) {
                        false => {
                            l = c;
                        }
                        true => {
                            r = c;
                        }
                    }
                }
                r
            }
        }
    }

    pub trait Sorted: Partitioned
    where
        Self::Item: Ord,
    {
        fn lower_bound(&self, x: &Self::Item) -> usize {
            self.partition_point(|y| x <= y)
        }

        fn upper_bound(&self, x: &Self::Item) -> usize {
            self.partition_point(|y| x < y)
        }
    }

    impl<T: Partitioned + ?Sized> Sorted for T where T::Item: Ord {}
}
// }}}
use slicetools::Sorted;

fn main() {
    input!(n: usize, a: [u32; n]);
    let mut dp = vec![std::u32::MAX; n + 1];
    for x in a {
        let i = dp.lower_bound(&x);
        dp[i] = x;
    }
    println!("{}", dp.lower_bound(&std::u32::MAX));
}

#[cfg(test)]
mod samples {
    const BIN: &str = "chap2_3_longest_increasing_sequence_fast";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"5
4 2 3 1 5
"#,
            "3\n",
        );
    }
}
