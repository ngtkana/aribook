use proconio::input;

fn main() {
    input!(n: usize, wv: [(usize, u32); n], cap: usize);
    let mut dp = vec![0; cap + 1];
    for &(w, v) in &wv {
        for j in (w..=cap).rev() {
            dp[j] = dp[j].max(v + dp[j - w]);
        }
    }
    println!("{}", dp[cap]);
}

#[cfg(test)]
mod samples {
    const BIN: &str = "chap2_3_01_knapsack_inline";

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
