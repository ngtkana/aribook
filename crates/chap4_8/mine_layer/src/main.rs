#![allow(clippy::many_single_char_names)]
use proconio::input;

fn sum(a: &[u32]) -> u32 {
    match a.len() % 3 {
        0 => a[1..].iter().step_by(3).sum(),
        1 | 2 => a[0..].iter().step_by(3).sum(),
        _ => unreachable!(),
    }
}

fn mid(a: &[u32]) -> u32 {
    let n = a.len();
    assert!(n % 2 == 1);
    let left = &a[..n / 2];
    let mut right = a[n / 2 + 1..].to_vec();
    right.reverse();
    let halves = [left, &right];

    match n % 3 {
        0 => {
            halves
                .iter()
                .map(|v| v[0..].iter().step_by(3))
                .flatten()
                .sum::<u32>()
                - sum(&a)
        }
        1 => {
            sum(&a)
                - halves
                    .iter()
                    .map(|v| v[1..].iter().step_by(3))
                    .flatten()
                    .sum::<u32>()
        }
        2 => {
            sum(&a)
                - halves
                    .iter()
                    .map(|v| v[0..].iter().step_by(3))
                    .flatten()
                    .sum::<u32>()
        }
        _ => unreachable!(),
    }
}

fn main() {
    input!(h: usize, w: usize, a: [[u32; w]; h]);
    let a = a.iter().map(|v| sum(v)).collect::<Vec<_>>();
    let ans = mid(&a);
    println!("{}", ans);
}

#[cfg(test)]
mod chap4_8_mine_layer_tests {
    const BIN: &str = "chap4_8_mine_layer";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3 3
2 2 1
3 4 1
2 3 2
"#,
            "1\n",
        );
    }
}
