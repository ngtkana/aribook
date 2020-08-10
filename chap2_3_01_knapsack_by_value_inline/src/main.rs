use proconio::input;

fn main() {
    input!(n: usize, wv: [(u32, usize); n], cap: u32);
    let sum_of_value = wv.iter().map(|&(_, v)| v).sum::<usize>();
    let mut dp = vec![std::u32::MAX; sum_of_value + 1];
    dp[0] = 0;
    for &(w, v) in &wv {
        for j in (v..=sum_of_value).rev() {
            dp[j] = dp[j].min(w.saturating_add(dp[j - v]))
        }
    }
    println!(
        "{}",
        dp.iter().enumerate().rfind(|(_, &w)| w <= cap).unwrap().0
    );
}

#[cfg(test)]
mod samples {
    const BIN: &str = "chap2_3_01_knapsack_by_value_inline";

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
