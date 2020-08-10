use proconio::input;

fn main() {
    input!(n: usize, m: usize, a: [usize; n], modulus: u32);
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for (i, &x) in a.iter().enumerate() {
        dp[i + 1][0] = 1;
        for j in 1..=m {
            dp[i + 1][j] = if j < x {
                (dp[i + 1][j - 1] + dp[i][j]) % modulus
            } else {
                (dp[i + 1][j - 1] + dp[i][j] + modulus - dp[i][j - x]) % modulus
            };
        }
    }
    println!("{}", dp[n][m]);
}

#[cfg(test)]
mod samples {
    const BIN: &str = "chap2_3_number_of_bounded_multichoice";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3 3
1 2 3
10000
"#,
            "6\n",
        );
    }
}
