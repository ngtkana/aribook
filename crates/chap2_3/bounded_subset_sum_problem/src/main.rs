use proconio::input;

fn main() {
    input!(n: usize, a: [usize; n], m: [usize; n], sum: usize);
    let mut dp = vec![vec![None; sum + 1]; n + 1];
    dp[0][0] = Some(0);
    for (i, &x) in a.iter().enumerate() {
        for j in 0..=sum {
            dp[i + 1][j] = if dp[i][j].is_some() {
                Some(m[i])
            } else {
                j.checked_sub(x)
                    .and_then(|j_minus_x| dp[i + 1][j_minus_x])
                    .and_then(|prv| prv.checked_sub(1))
            };
        }
    }
    println!("{}", if dp[n][sum].is_some() { "Yes" } else { "No" });
}

#[cfg(test)]
mod samples {
    const BIN: &str = "chap2_3_bounded_subset_sum_problem";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3
3 5 8
3 2 2
17
"#,
            "Yes\n",
        );
    }
}
