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
use itertools::Itertools;
use ordtools::Ordtools;
use proconio::input;

fn div_conq(a: &mut [(i64, i64)]) -> i64 {
    let n = a.len();
    if n <= 1 {
        std::i64::MAX
    } else {
        // 分割パート
        let (left, right) = a.split_at_mut(n / 2);
        let split_x = right.first().unwrap().0;
        let mut ans = div_conq(left).min(div_conq(right));
        let swp = left
            .iter()
            .merge_by(right.iter(), |(_, y0), (_, y1)| y0 < y1)
            .cloned()
            .collect::<Vec<_>>();
        a.copy_from_slice(swp.as_slice());

        // 合体パート
        let near_points = a
            .iter()
            .filter(|&&(x, _)| {
                let dx = (x - split_x).abs().pow(2);
                dx * dx < ans
            })
            .collect::<Vec<_>>();
        for (i, (x0, y0)) in near_points.iter().enumerate() {
            for (x1, y1) in &near_points[i + 1..] {
                let dy = (y1 - y0).abs();
                if ans < dy * dy {
                    break;
                }
                let dx = (x1 - x0).abs();
                ans.change_min(dx * dx + dy * dy);
            }
        }
        ans
    }
}

fn main() {
    input!(n: usize, mut xy: [(i64, i64); n]);
    xy.sort();
    let ans = div_conq(&mut xy);

    println!("{:.4}", (ans as f64).sqrt());
}

#[cfg(test)]
mod chap4_6_nearest_point_pair_tests {
    const BIN: &str = "chap4_6_nearest_point_pair";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"5
0 2
6 67
43 71
39 107
189 140
"#,
            "36.2215\n",
        );
    }
}
