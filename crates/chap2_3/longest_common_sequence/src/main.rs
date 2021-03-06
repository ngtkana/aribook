use proconio::{input, marker::Chars};
fn main() {
    input!(n: usize, m: usize, a: Chars, b: Chars);
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            dp[i + 1][j + 1] = if x == y {
                dp[i][j] + 1
            } else {
                dp[i + 1][j].max(dp[i][j + 1])
            }
        }
    }
    println!("{}", dp[n][m]);
}

#[cfg(test)]
mod samples {
    const BIN: &str = "chap2_3_longest_common_sequence";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"4 4
abcd
becd
"#,
            "3\n",
        );
    }
}
