#![allow(clippy::many_single_char_names)]
use chap3_5_minimum_cost_flow_dijkstra::{MinCostFlowDijkstra, MinCostFlowResult};
use proconio::{input, marker::Usize1};

fn main() {
    input!(n: usize, k: i32, abw: [(Usize1, Usize1, i32); n]);
    let line_length = abw.iter().map(|&(_, b, _)| b).max().unwrap() + 1;
    let mut demand = vec![0; line_length];
    demand[0] = -k;
    demand[line_length - 1] = k;
    let s = line_length;
    let t = line_length + 1;
    let mut mcf = MinCostFlowDijkstra::with_len(line_length + 2);
    (0..line_length - 1).for_each(|i| mcf.add_edge(i, i + 1, std::u32::MAX, 0));
    let mut cost_shift = 0;
    for (a, b, w) in abw {
        mcf.add_edge(b, a, 1, w);
        demand[a] += 1;
        demand[b] -= 1;
        cost_shift += w;
    }
    for (i, &d) in demand.iter().enumerate() {
        use std::cmp::Ordering;
        match &d.cmp(&0) {
            Ordering::Less => mcf.add_edge(s, i, (-d) as u32, 0),
            Ordering::Greater => mcf.add_edge(i, t, d as u32, 0),
            Ordering::Equal => (),
        }
    }

    let required_flow = demand.iter().filter(|&&x| 0 < x).sum::<i32>();
    match mcf.run(s, t, required_flow as u32) {
        MinCostFlowResult::Cost(cost) => {
            let ans = -(cost as i32 - cost_shift as i32);
            println!("{}", ans);
        }
        MinCostFlowResult::Impossible => unreachable!(),
    }
}

#[cfg(test)]
mod chap3_5_intervals_tests {
    const BIN: &str = "chap3_5_intervals";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3 1
1 2 2
2 3 4
3 4 8
"#,
            "14\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"3 1
1 3 2
2 3 4
3 4 8
"#,
            "12\n",
        );
    }

    #[test]
    fn sample3() {
        test_sample(
            r#"3 2
1 100000 100000
1 150 301

100 200 300
"#,
            "100301\n",
        );
    }
}
