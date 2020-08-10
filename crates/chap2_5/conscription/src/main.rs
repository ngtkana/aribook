use proconio::input;
use std::cmp::Reverse;

fn main() {
    input!(n: usize, m: usize, q: usize, uvw: [(usize, usize, u64); q]);
    let mut g = vec![vec![]; n + m];
    for (u, v, w) in uvw {
        let v = n + v;
        g[u].push((v, 10_000 - w));
        g[v].push((u, 10_000 - w));
    }
    let n = n + m;
    let mut cost = 0;
    let mut used = vec![false; n];
    let mut heap = std::collections::BinaryHeap::new();
    for start in 0..n {
        if used[start] {
            continue;
        }
        heap.push((Reverse(10_000), start));
        while let Some((Reverse(dx), x)) = heap.pop() {
            if std::mem::replace(&mut used[x], true) {
                continue;
            }
            cost += dx;
            for &(y, w) in g[x].iter().filter(|&&(y, _)| !used[y]) {
                heap.push((Reverse(w), y));
            }
        }
    }
    println!("{}", cost);
}

#[cfg(test)]
mod chap2_5_conscription_tests {
    const BIN: &str = "chap2_5_conscription";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"5 5 8
4 3 6831
1 3 4583
0 0 6592
0 1 3063
3 3 4975
1 3 2049
4 2 2104
2 2 781
"#,
            "71071\n",
        );
    }
}
