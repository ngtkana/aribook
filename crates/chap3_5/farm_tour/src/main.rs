#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input!(n: usize, m: usize, abc: [(Usize1, Usize1, i32); m]);
    let mut mcf = chap3_5_minimum_cost_flow_dijkstra::MinCostFlowDijkstra::with_len(n);
    for (a, b, c) in abc {
        mcf.add_edge(a, b, 1, c);
        mcf.add_edge(b, a, 1, c);
    }
    let s = 0;
    let t = n - 1;
    match mcf.run(s, t, 2) {
        chap3_5_minimum_cost_flow_dijkstra::MinCostFlowResult::Cost(ans) => println!("{}", ans),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod chap3_5_farm_tour_tests {
    const BIN: &str = "chap3_5_farm_tour";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"4 5
1 2 1
2 3 1
3 4 1
1 3 2
2 4 2
"#,
            "6\n",
        );
    }
}
