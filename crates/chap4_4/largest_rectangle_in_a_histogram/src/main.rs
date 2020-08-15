#![allow(clippy::many_single_char_names)]
use proconio::input;

fn main() {
    input!(n: usize, a: [u64; n]);
    let mut left = vec![0; n];
    let mut right = vec![n; n];
    let mut stack = std::collections::VecDeque::new();
    for (i, &x) in a.iter().enumerate() {
        while let Some(j) = stack.pop_back() {
            if a[j] < x {
                left[i] = j + 1;
                stack.push_back(j);
                break;
            }
        }
        stack.push_back(i);
    }
    stack.clear();
    for (i, &x) in a.iter().enumerate().rev() {
        while let Some(j) = stack.pop_back() {
            if a[j] < x {
                right[i] = j;
                stack.push_back(j);
                break;
            }
        }
        stack.push_back(i);
    }
    println!(
        "{}",
        left.iter()
            .zip(right.iter())
            .enumerate()
            .map(|(i, (&l, &r))| {
                assert!(l <= i && i < r);
                (r - l) as u64 * a[i]
            })
            .max()
            .unwrap()
    );
}

#[cfg(test)]
mod chap4_4_largest_rectangle_in_a_histogram_tests {
    const BIN: &str = "chap4_4_largest_rectangle_in_a_histogram";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"7
2 1 4 5 1 3 3
"#,
            "8\n",
        );
    }
}
