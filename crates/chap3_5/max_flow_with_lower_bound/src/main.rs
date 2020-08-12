#![allow(clippy::many_single_char_names)]
// dbg {{{
#[allow(dead_code)]
mod dbg {
    use std::fmt::{Debug, Formatter};

    #[derive(Clone)]
    pub struct Tabular<'a, T: Debug>(pub &'a [T]);
    impl<'a, T: Debug> Debug for Tabular<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            for i in 0..self.0.len() {
                writeln!(f, "{:2} | {:?}", i, &self.0[i])?;
            }
            Ok(())
        }
    }

    #[derive(Clone)]
    pub struct BooleanTable<'a>(pub &'a [Vec<bool>]);
    impl<'a> Debug for BooleanTable<'a> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            for i in 0..self.0.len() {
                writeln!(f, "{:2} | {:?}", i, BooleanSlice(&self.0[i]))?;
            }
            Ok(())
        }
    }

    #[derive(Clone)]
    pub struct BooleanSlice<'a>(pub &'a [bool]);
    impl<'a> Debug for BooleanSlice<'a> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(
                f,
                "{}",
                self.0
                    .iter()
                    .map(|&b| if b { "1 " } else { "0 " })
                    .collect::<String>()
            )?;
            Ok(())
        }
    }
}
// }}}

use proconio::input;

#[derive(Debug, Clone)]
pub struct Edge {
    to: usize,
    cap: i32,
    rev: usize,
}

#[derive(Debug, Clone)]
pub struct FordFulkerson {
    graph: Vec<Vec<Edge>>,
}

impl FordFulkerson {
    pub fn with_len(len: usize) -> Self {
        Self {
            graph: vec![Vec::new(); len],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: i32) {
        let from_len = self.graph[from].len();
        let to_len = self.graph[to].len();
        self.graph[from].push(Edge {
            to,
            cap,
            rev: to_len,
        });
        self.graph[to].push(Edge {
            to: from,
            cap: 0,
            rev: from_len,
        });
    }

    pub fn max_flow(&mut self, s: usize, t: usize) -> i32 {
        fn dfs(
            x: usize,
            t: usize,
            f: i32,
            used: &mut [bool],
            g: &[Vec<Edge>],
            path: &mut Vec<usize>,
        ) -> i32 {
            if x == t {
                return f;
            }
            used[x] = true;
            for (i, &Edge { to, cap, rev: _ }) in g[x].iter().enumerate() {
                if !used[to] && 0 != cap {
                    let d = dfs(to, t, f.min(cap), used, g, path);
                    if d != 0 {
                        path.push(i);
                        return d;
                    }
                }
            }
            0
        }
        let mut flow = 0;
        loop {
            let mut used = vec![false; self.graph.len()];
            let mut path = Vec::new();
            let f = dfs(s, t, std::i32::MAX, &mut used, &self.graph, &mut path);
            if f == 0 {
                return flow;
            }
            let mut now = s;
            for &i in path.iter().rev() {
                let (to, rev) = {
                    let Edge { to, cap, rev } = &mut self.graph[now][i];
                    *cap -= f;
                    (*to, *rev)
                };
                self.graph[to][rev].cap += f;
                now = to;
            }
            assert_eq!(now, t);
            flow += f;
        }
    }
}

fn main() {
    input!(
        n: usize,
        m: usize,
        s: usize,
        t: usize,
        uvlr: [(usize, usize, i32, i32); m]
    );
    let super_s = n;
    let super_t = n + 1;
    let mut ff = FordFulkerson::with_len(n + 2);
    let mut demand = vec![0; n];
    for (u, v, l, r) in uvlr {
        demand[u] -= l;
        demand[v] += l;
        if l < r {
            ff.add_edge(u, v, r - l);
        }
    }
    ff.add_edge(t, s, std::i32::MAX);
    let mut sum_of_demands = 0;
    for (i, &x) in demand.iter().enumerate() {
        if 0 < x {
            sum_of_demands += x;
            ff.add_edge(super_s, i, x);
        }
        if x < 0 {
            ff.add_edge(i, super_t, -x);
        }
    }
    let demand_flow = ff.max_flow(super_s, super_t);

    if demand_flow != sum_of_demands {
        println!("Impossible");
        std::process::exit(0);
    }

    let main_flow = ff.max_flow(s, t);
    println!("{}", main_flow);
}

#[cfg(test)]
mod chap3_5_max_flow_with_lower_bound_tests {
    const BIN: &str = "chap3_5_max_flow_with_lower_bound";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn test_hand_possible_1() {
        test_sample(
            r#"5 7 0 4
0 1 0 10
0 2 0 2
1 2 5 6
1 3 0 6
2 4 0 5
3 2 0 3
3 4 0 8
"#,
            "10\n",
        );
    }

    #[test]
    fn test_hand_possible_2() {
        test_sample(
            r#"5 7 0 4
0 1 0 10
0 2 0 2
1 2 0 6
1 3 0 6
2 4 0 5
3 2 2 3
3 4 0 8
"#,
            "9\n",
        );
    }

    #[test]
    fn test_hand_possible_3() {
        test_sample(
            r#"3 2 0 2
0 1 1 2
1 2 2 3
"#,
            "2\n",
        );
    }

    #[test]
    fn test_hand_impossible() {
        test_sample(
            r#"5 7 0 4
0 1 0 10
0 2 0 2
1 2 0 6
1 3 0 6
2 4 0 5
3 2 2 3
3 4 5 8
"#,
            "Impossible\n",
        );
    }
}
