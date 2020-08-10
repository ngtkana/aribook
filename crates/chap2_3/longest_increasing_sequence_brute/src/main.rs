use proconio::input;

fn main() {
    input!(n: usize, a: [u32; n]);
    let mut dp = vec![0; n];
    dp[0] = 1;
    for (i, &ai) in a.iter().enumerate() {
        dp[i] = dp[0..i]
            .iter()
            .enumerate()
            .filter_map(|(j, &dp_j)| if a[j] < ai { Some(dp_j + 1) } else { None })
            .max()
            .unwrap_or(1);
    }
    println!("{}", dp[n - 1]);
}

#[cfg(test)]
mod samples {
    const BIN: &str = "chap2_3_longest_increasing_sequence_brute";

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
