#![allow(clippy::many_single_char_names)]
use chap4_3_binary_indexed_tree::BinaryIndexedTree;
use chap4_3_lca_rmq::LcaRmq;
use proconio::{input, marker::Usize1};

fn main() {
    input!(
        n: usize,
        q: usize,
        mut now: Usize1,
        abw: [(Usize1, Usize1, i64); n - 1]
    );

    let mut graph = vec![vec![]; n];
    for (i, &(a, b, w)) in abw.iter().enumerate() {
        graph[a].push((i, b, w));
        graph[b].push((i, a, w));
    }

    fn dfs(
        x: usize,
        p: usize,
        d: u32,
        edge_order: &mut Vec<usize>,
        weight: &mut Vec<i64>,
        graph: &[Vec<(usize, usize, i64)>],
    ) {
        for &(i, y, w) in graph[x].iter().filter(|&&(_, y, _)| y != p) {
            edge_order.push(i);
            weight.push(w);
            dfs(y, x, d + 1, edge_order, weight, graph);
            edge_order.push(i);
            weight.push(-w);
        }
    }
    let root = 0;
    let mut edge_order = Vec::new();
    let mut weight = Vec::new();
    dfs(root, root, 0, &mut edge_order, &mut weight, &graph);

    let mut edge_pos = vec![vec![]; n - 1];
    for (i, &x) in edge_order.iter().enumerate() {
        edge_pos[x].push(i);
    }

    let mut binary_indexed_tree = BinaryIndexedTree::with_slice(&weight);
    let lca = LcaRmq::with_graph(
        root,
        &graph
            .iter()
            .map(|v| v.iter().map(|&(_, y, _)| y).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );

    for _ in 0..q {
        input!(command: char);
        match command {
            'A' => {
                input!(next: Usize1);
                let u = now;
                let v = next;
                let q = lca.query(u, v);
                let mut u = lca.start[now];
                let mut v = lca.start[next];
                let q = lca.start[q];
                if u > v {
                    std::mem::swap(&mut u, &mut v);
                }
                assert!((u..=v).contains(&q));
                let x = binary_indexed_tree.range_sum(u..q).unwrap_or(0);
                let y = binary_indexed_tree.range_sum(q..v).unwrap_or(0);
                println!("{}", -x - y);
                now = next;
            }
            'B' => {
                input!(edge_id: Usize1, w: i64);
                match *edge_pos[edge_id].as_slice() {
                    [x, y] => {
                        binary_indexed_tree.update(x, w);
                        binary_indexed_tree.update(y, -w);
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod chap4_3_housewife_wind_tests {
    const BIN: &str = "chap4_3_housewife_wind";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3 3 1
1 2 1
2 3 2
A 2
B 2 3
A 3
"#,
            r#"1
3
"#,
        );
    }
}
