#![allow(clippy::many_single_char_names)]
use chap3_5_minimum_cost_flow_dijkstra::{MinCostFlowDijkstra, MinCostFlowResult};
use proconio::input;

fn main() {
    input!(
        n: usize,
        m: usize,
        s: usize,
        t: usize,
        f: i32,
        uvcd: [(usize, usize, u32, i32); m]
    );
    let super_s = n;
    let super_t = n + 1;
    let mut mcf = MinCostFlowDijkstra::with_len(n + 2);
    let mut demand = vec![0; n];
    demand[s] += f;
    demand[t] -= f;
    let mut cost_shift = 0;
    for (u, v, c, d) in uvcd {
        if 0 <= d {
            mcf.add_edge(u, v, c, d);
        } else {
            let d_abs = -d;
            cost_shift += d_abs as u32 * c;
            demand[u] -= c as i32;
            demand[v] += c as i32;
            mcf.add_edge(v, u, c, d_abs);
        }
    }
    for (i, &d) in demand.iter().enumerate() {
        use std::cmp::Ordering;
        match d.cmp(&0) {
            Ordering::Less => mcf.add_edge(i, super_t, (-d) as u32, 0),
            Ordering::Greater => mcf.add_edge(super_s, i, d as u32, 0),
            Ordering::Equal => (),
        }
    }
    let required = demand.iter().filter(|&&x| 0 < x).sum::<i32>() as u32;
    match mcf.run(super_s, super_t, required) {
        MinCostFlowResult::Cost(cost) => {
            println!("{}", cost as i32 - cost_shift as i32);
        }
        MinCostFlowResult::Impossible => {
            println!("Impossible");
        }
    }
}

#[cfg(test)]
mod chap3_5_minimum_cost_flow_remove_negative_edge_tests {
    const BIN: &str = "chap3_5_minimum_cost_flow_remove_negative_edge";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn test_hand1() {
        test_sample(
            r#"2 1 0 1 10
0 1 5 -10
"#,
            "Impossible\n",
        );
    }

    #[test]
    fn test_hand2() {
        test_sample(
            r#"2 1 0 1 10
0 1 15 -10
"#,
            "-100\n",
        );
    }

    #[test]
    fn test_hand3() {
        test_sample(
            r#"2 2 0 1 10
0 1 7 -10
0 1 7 -20
"#,
            "-170\n",
        );
    }

    #[test]
    fn test_hand4() {
        test_sample(
            r#"4 5 0 3 8
0 1 10 10
1 2 10 -12
2 3 10 10
1 3 1 -1
1 3 1 -3
"#,
            "63\n",
        );
    }
}
