use proconio::input;

fn main() {
    input!(n: usize, wv: [(usize, u32); n], cap: usize);
    let mut dp = vec![vec![0; cap as usize + 1]; n + 1];
    for (i, &(w, v)) in wv.iter().enumerate().rev() {
        for j in 0..=cap {
            dp[i][j] = if j < w {
                dp[i + 1][j]
            } else {
                dp[i + 1][j].max(v + dp[i + 1][j - w])
            }
        }
    }
    println!("{}", dp[0][cap]);
}

#[cfg(test)]
mod samples {
    const BIN: &str = "chap2_3_01knapsack_dp";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"4
2 3
1 2
3 4
2 2
5
"#,
            "7\n",
        );
    }
}
