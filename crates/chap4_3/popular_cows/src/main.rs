#![allow(clippy::many_single_char_names)]
use chap4_3_scc::Scc;
use proconio::{input, marker::Usize1};

fn main() {
    input!(n: usize, m: usize, ab: [(Usize1, Usize1); m]);
    let mut scc = Scc::with_len(n);
    let mut reverse_graph = vec![vec![]; n];
    for (a, b) in ab {
        scc.add_edge(a, b);
        reverse_graph[b].push(a);
    }
    let cmp = scc.run();
    let start = *cmp.last().unwrap().first().unwrap();
    let mut used = vec![false; n];
    let mut stack = std::collections::VecDeque::from(vec![start]);
    while let Some(x) = stack.pop_back() {
        used[x] = true;
        for &y in reverse_graph[x].iter().filter(|&&y| !used[y]) {
            stack.push_back(y);
        }
    }
    let ans = if used.iter().all(|&x| x) {
        cmp.last().unwrap().len()
    } else {
        0
    };
    println!("{}", ans);
}

#[cfg(test)]
mod chap4_3_popular_cows_tests {
    const BIN: &str = "chap4_3_popular_cows";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3 3
1 2
2 1
2 3
"#,
            "1\n",
        );
    }
}
