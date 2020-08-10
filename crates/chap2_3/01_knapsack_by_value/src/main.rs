use proconio::input;

fn main() {
    input!(n: usize, wv: [(u32, usize); n], cap: u32);
    let sum_of_value = wv.iter().map(|&(_, v)| v).sum::<usize>();
    let mut dp = vec![vec![std::u32::MAX; sum_of_value + 1]; n + 1];
    dp[0][0] = 0;
    for (i, &(w, v)) in wv.iter().enumerate() {
        for j in 0..=sum_of_value {
            dp[i + 1][j] = if j < v {
                dp[i][j]
            } else {
                dp[i][j].min(w.saturating_add(dp[i][j - v]))
            };
        }
    }
    println!(
        "{}",
        dp[n]
            .iter()
            .enumerate()
            .rfind(|(_, &w)| w <= cap)
            .unwrap()
            .0
    );
}

#[cfg(test)]
mod samples {
    const BIN: &str = "chap2_3_01_knapsack_by_value";

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
