use proconio::input;

fn main() {
    input!(
        n: usize,
        m: usize,
        xyb: [(i32, i32, i32); n],
        pqc: [(i32, i32, i32); m],
        mut e: [[i32; m]; n],
    );
    let graph_size = n + m + 1;
    let mut dist = vec![vec![std::i32::MAX; graph_size]; graph_size];
    (0..graph_size).for_each(|i| dist[i][i] = 0);
    for (j, &(p, q, c)) in pqc.iter().enumerate() {
        for (i, &(x, y, ..)) in xyb.iter().enumerate() {
            let cost = (p - x).abs() + (q - y).abs() + 1;
            dist[i][n + j] = cost;
            if 0 < e[i][j] {
                dist[n + j][i] = -cost;
            }
        }
        let occupied = e.iter().map(|v| v[j]).sum::<i32>();
        if 0 != occupied {
            dist[n + m][n + j] = 0;
        }
        if occupied != c {
            dist[n + j][n + m] = 0;
        }
    }
    let mut prev = (0..graph_size)
        .map(|i| vec![i; graph_size])
        .collect::<Vec<_>>();
    for k in 0..graph_size {
        for i in 0..graph_size {
            for j in 0..graph_size {
                let nd = dist[i][k].saturating_add(dist[k][j]);
                if nd < dist[i][j] {
                    dist[i][j] = nd;
                    prev[i][j] = prev[k][j];
                    if i == j && dist[i][i] < 0 {
                        let mut used = vec![false; graph_size];
                        let mut now = i;
                        while !std::mem::replace(&mut used[now], true) {
                            let nxt = prev[i][now];
                            if now != n + m && nxt != n + m {
                                if now <= n {
                                    e[now][nxt - n] -= 1;
                                } else {
                                    e[nxt][now - n] += 1;
                                }
                            }
                            now = nxt;
                        }
                        println!("SUBOPTIMAL");
                        for v in e {
                            println!(
                                "{}",
                                v.iter()
                                    .map(|x| x.to_string())
                                    .fold(String::new(), |acc, x| if acc.is_empty() {
                                        x
                                    } else {
                                        acc + " " + x.as_ref()
                                    })
                            );
                        }
                        std::process::exit(0);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod chap3_5_evacuation_plan_floyd_warshall_tests {
    const BIN: &str = "chap3_5_evacuation_plan_floyd_warshall";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3 4
-3 3 5
-2 -2 6
2 2 5
-1 1 3
1 1 4
-2 -2 7
0 -1 3
3 1 1 0
0 0 6 0
0 3 0 2
"#,
            r#"SUBOPTIMAL
3 0 1 1
0 0 6 0
0 4 0 1
"#,
        );
    }
}
