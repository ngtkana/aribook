use proconio::input;

fn main() {
    input!(
        n: usize,                      // 頂点数
        m: usize,                      // 辺の本数
        a: [u32; n],                   // 頂点の容量
        uvw: [(usize, usize, u32); m]  // 辺
    );
    let mut ff = chap3_5_ford_fulkerson::FordFulkerson::with_len(2 * n);
    for (i, &ai) in a.iter().enumerate() {
        ff.add_edge(i, i + n, ai);
    }
    for (u, v, w) in uvw {
        ff.add_edge(u + n, v, w);
    }
    println!("{}", ff.max_flow(0, 4));
}

#[cfg(test)]
mod chap3_5_capacity_of_verticies_tests {
    const BIN: &str = "chap3_5_capacity_of_verticies";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn hand() {
        test_sample(
            r#"5 7
20 20 4 20 20
0 1 10
0 2 2
1 2 6
1 3 6
2 4 5
3 2 3
3 4 8
"#,
            "10\n",
        );
    }
}
