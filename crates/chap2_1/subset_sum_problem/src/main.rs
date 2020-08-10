use proconio::input;

fn main() {
    input!(n: usize, a: [u32; n], k: u32);
    fn dfs(a: &[u32]) -> Vec<u32> {
        if a.is_empty() {
            vec![0]
        } else {
            let ans = dfs(&a[1..]);
            ans.iter()
                .copied()
                .chain(ans.iter().copied().map(|x| x + a[0]))
                .collect()
        }
    }
    let ans = dfs(&a);
    println!(
        "{}",
        if ans.iter().find(|&&x| x == k).is_some() {
            "Yes"
        } else {
            "No"
        }
    );
}

#[cfg(test)]
mod samples {
    const BIN: &str = "chap2_1_subset_sum_problem";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"4
1 2 4 7
13
"#,
            "Yes\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"4
1 2 4 7
15
"#,
            "No\n",
        );
    }
}
