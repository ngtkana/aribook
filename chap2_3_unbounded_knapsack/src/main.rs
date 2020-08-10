use proconio::input;

fn main() {
    input!(n: usize, wv: [(usize, u32); n], cap: usize);
    let mut dp = vec![vec![0; cap + 1]; n + 1];
    for (i, &(w, v)) in wv.iter().enumerate() {
        for j in 0..=cap {
            dp[i + 1][j] = if j < w {
                dp[i][j]
            } else {
                dp[i][j].max(v + dp[i + 1][j - w])
            };
        }
    }
    println!("{}", dp[n][cap]);
}

#[cfg(test)]
mod samples {
    const BIN: &str = "chap2_3_unbounded_knapsack";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3
3 4
4 5
2 3
7
"#,
            "10\n",
        );
    }
}
