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
    input!(n: usize, a: [u32; n]);
    let all_kind = a
        .iter()
        .copied()
        .collect::<std::collections::HashSet<_>>()
        .len();
    let mut map = std::collections::HashMap::new();
    let mut ans = n;
    let mut kind = 0;
    let mut r = 0;
    for l in 0..n {
        while r < n && kind < all_kind {
            let z = map.entry(&a[r]).or_insert(0);
            if *z == 0 {
                kind += 1;
            }
            *z += 1;
            r += 1;
            if all_kind == kind {
                break;
            }
        }
        if all_kind != kind {
            break;
        }
        ans.change_min(r - l);
        let z = map.entry(&a[l]).or_insert(0);
        assert!(0 <= *z);
        *z -= 1;
        if *z == 0 {
            kind += 1;
        }
    }
    println!("{}", ans);
}

#[cfg(test)]
mod chap3_2_jessicas_reading_problem_tests {
    const BIN: &str = "chap3_2_jessicas_reading_problem";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"5
1 8 8 8 1
"#,
            "2\n",
        );
    }
}
