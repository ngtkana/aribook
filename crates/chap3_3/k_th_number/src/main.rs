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
use proconio::{input, marker::Usize1};
use slicetools::Sorted;

#[allow(clippy::many_single_char_names)]
fn main() {
    input!(n: usize, q: usize, a: [u32; n]);
    let a_max = *a.iter().max().unwrap();
    let block_size = (n as f64 * n.next_power_of_two().trailing_zeros() as f64).sqrt() as usize;
    let mut sorted = a.clone();
    sorted.chunks_exact_mut(block_size).for_each(<[_]>::sort);
    for _ in 0..q {
        // center 以上のものが want = r - l - k 個以上あるような最高の center を計算します。
        input!(l: Usize1, r: usize, k: Usize1);
        let want = r - l - k;
        let mut lower = 0;
        let mut upper = a_max + 1;
        while 1 < upper - lower {
            let center = lower + (upper - lower) / 2;
            let mut l = l;
            let mut r = r;
            let mut count = 0;
            while l < r && l % block_size != 0 {
                if center <= a[l] {
                    count += 1;
                }
                l += 1;
            }
            while l < r && r % block_size != 0 {
                r -= 1;
                if center <= a[r] {
                    count += 1;
                }
            }
            while l < r {
                count += block_size - sorted[l..l + block_size].lower_bound(&center);
                l += block_size;
            }
            if want <= count {
                lower = center;
            } else {
                upper = center;
            }
        }
        println!("{}", lower);
    }
}

#[cfg(test)]
mod chap3_3_k_th_number_tests {
    const BIN: &str = "chap3_3_k_th_number";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"7 3
1 5 2 6 3 7 4
2 5 3
4 4 1
1 7 3
"#,
            r#"5
6
3
"#,
        );
    }
}
