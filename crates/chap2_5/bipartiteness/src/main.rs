use proconio::input;

fn main() {
    input!(n: usize, m: usize, uv: [(usize, usize); m]);
    let mut g = vec![vec![]; n];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
    }
    fn dfs(x: usize, cx: bool, color: &mut [Option<bool>], g: &[Vec<usize>]) -> bool {
        color[x] = Some(cx);
        for &y in &g[x] {
            if let Some(cy) = color[y] {
                if cx == cy {
                    return false;
                }
            } else if !dfs(y, !cx, color, g) {
                return false;
            }
        }
        true
    }
    let mut ans = true;
    let mut color = vec![None; n];
    for i in 0..n {
        if color[i].is_none() {
            ans &= dfs(i, false, &mut color, &g);
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}

#[cfg(test)]
mod chap2_5_bipartiteness_tests {
    const BIN: &str = "chap2_5_bipartiteness";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3 3
0 1
0 2
1 2
"#,
            "No\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"4 4
0 1
0 3
1 2
2 3
"#,
            "Yes\n",
        );
    }
}
