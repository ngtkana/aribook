use proconio::input;

fn main() {
    input!(n: usize, m: usize, modulus: u32);
    let mut dp = vec![vec![0; n + 1]; m + 1];
    dp[0][0] = 1;
    for i in 1..=m {
        for j in 0..=n {
            dp[i][j] = if i <= j {
                (dp[i - 1][j] + dp[i][j - i]) % modulus
            } else {
                dp[i - 1][j]
            };
        }
    }
    println!("{}", dp[m][n]);
}

#[cfg(test)]
mod samples {
    const BIN: &str = "chap2_3_number_of_partitions";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"4 3 10000
"#,
            "4\n",
        );
    }
}
