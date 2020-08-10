use proconio::input;

fn main() {
    input!(n: usize, wv: [(u32, u32); n], cap: u32);
    let mut memo = vec![vec![0; n + 1]; cap as usize + 1];
    fn dfs(vacancy: u32, wv: &[(u32, u32)], memo: &mut [Vec<u32>]) -> u32 {
        let here = memo[vacancy as usize][wv.len()];
        if here != 0 {
            here
        } else {
            let ans = if wv.is_empty() {
                0
            } else {
                let (w, v) = wv[0];
                let rest = &wv[1..];
                if vacancy < w {
                    dfs(vacancy, rest, memo)
                } else {
                    dfs(vacancy, rest, memo).max(v + dfs(vacancy - w, rest, memo))
                }
            };
            memo[vacancy as usize][wv.len()] = ans;
            ans
        }
    }
    println!("{}", dfs(cap, &wv, &mut memo));
}

#[cfg(test)]
mod samples {
    const BIN: &str = "chap2_3_01knapsack_recursive";

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
