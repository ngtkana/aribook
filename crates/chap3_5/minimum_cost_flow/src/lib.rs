#![allow(clippy::many_single_char_names)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Edge {
    to: usize,
    cap: u32,
    cost: i32,
    rev: usize,
}

#[derive(Clone, Eq, PartialEq)]
pub struct PrettyEdge {
    to: usize,
    flow: u32,
    cap: u32,
    cost: i32,
}

impl std::fmt::Debug for PrettyEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}/{}, {})",
            self.to, self.flow, self.cap, self.cost
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ShortestPathResult {
    Finite { path: Vec<usize>, cost: i32 },
    Unreachable,
    NegativeCycle,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MinCostFlowResult {
    Cost(i32),
    Impossible,
    NegativeCycle,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DistWithPredecessor {
    dist: i32,
    prv: usize,
    id: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MinCostFlow {
    graph: Vec<Vec<Edge>>,
    pub edge_keys: Vec<(usize, usize)>,
}

impl MinCostFlow {
    pub fn with_len(len: usize) -> Self {
        Self {
            graph: vec![Vec::new(); len],
            edge_keys: Vec::new(),
        }
    }

    pub fn pretty(&self) -> Vec<Vec<PrettyEdge>> {
        let hash_set = self
            .edge_keys
            .iter()
            .cloned()
            .collect::<std::collections::HashSet<_>>();
        self.graph
            .iter()
            .enumerate()
            .map(|(i, v)| {
                v.iter()
                    .enumerate()
                    .filter_map(|(j, &Edge { to, cap, rev, cost })| {
                        if hash_set.get(&(i, j)).is_some() {
                            let revcap = self.graph[to][rev].cap;
                            Some(PrettyEdge {
                                to,
                                flow: revcap,
                                cap: cap + revcap,
                                cost,
                            })
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect()
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: u32, cost: i32) {
        let from_len = self.graph[from].len();
        let to_len = self.graph[to].len();
        self.edge_keys.push((from, from_len));
        self.graph[from].push(Edge {
            to,
            cap,
            cost,
            rev: to_len,
        });
        self.graph[to].push(Edge {
            to: from,
            cap: 0,
            cost: -cost,
            rev: from_len,
        });
    }

    pub fn find_shortest_path_bellmann_ford(&self, s: usize, t: usize) -> ShortestPathResult {
        fn relax(me: &MinCostFlow, table: &mut [Option<DistWithPredecessor>]) -> bool {
            let mut update = false;
            for x in 0..me.graph.len() {
                if let Some(DistWithPredecessor { dist: dx, .. }) = table[x] {
                    for (i, &Edge { to: y, cost, .. }) in
                        me.graph[x].iter().enumerate().filter(|(_, e)| e.cap != 0)
                    {
                        let ndy = dx.saturating_add(cost);
                        let needs_update = table[y]
                            .as_ref()
                            .map(|&DistWithPredecessor { dist: dy, .. }| ndy < dy)
                            .unwrap_or(true);
                        if needs_update {
                            update = true;
                            table[y] = Some(DistWithPredecessor {
                                dist: ndy,
                                prv: x,
                                id: i,
                            });
                        }
                    }
                }
            }
            update
        }

        let mut table = vec![None; self.graph.len()];
        table[s] = Some(DistWithPredecessor {
            dist: 0,
            prv: s,
            id: 0,
        });

        if (0..self.graph.len())
            .map(|_| relax(&self, table.as_mut_slice()))
            .last()
            .unwrap()
        {
            ShortestPathResult::NegativeCycle
        } else if let Some(DistWithPredecessor { dist: cost, .. }) = table[t] {
            let mut now = t;
            let mut path = Vec::new();
            while now != s {
                let DistWithPredecessor {
                    prv: pnow,
                    id: inow,
                    ..
                } = table[now].as_ref().unwrap();
                path.push(*inow);
                now = *pnow;
            }
            ShortestPathResult::Finite { path, cost }
        } else {
            ShortestPathResult::Unreachable
        }
    }

    pub fn min_cap_flow_along_path(&mut self, s: usize, path: &[usize]) -> u32 {
        let mut now = s;
        path.iter()
            .rev()
            .map(|&i| {
                let Edge { to, cap, .. } = self.graph[now][i];
                now = to;
                cap
            })
            .min()
            .unwrap()
    }

    pub fn push_along_path(&mut self, s: usize, f: u32, path: &[usize]) {
        let mut now = s;
        for &i in path.iter().rev() {
            let (to, rev) = {
                let Edge { to, cap, rev, .. } = &mut self.graph[now][i];
                *cap -= f;
                (*to, *rev)
            };
            self.graph[to][rev].cap += f;
            now = to;
        }
    }

    pub fn run(&mut self, s: usize, t: usize, mut d: u32) -> MinCostFlowResult {
        if d == 0 {
            return MinCostFlowResult::Cost(0);
        }
        let mut total_cost = 0;
        loop {
            match self.find_shortest_path_bellmann_ford(s, t) {
                ShortestPathResult::Unreachable => {
                    return MinCostFlowResult::Impossible;
                }
                ShortestPathResult::NegativeCycle => {
                    return MinCostFlowResult::NegativeCycle;
                }
                ShortestPathResult::Finite { path, cost } => {
                    let f = self.min_cap_flow_along_path(s, &path).min(d);
                    d -= f;
                    total_cost += f as i32 * cost;
                    self.push_along_path(s, f, &path);
                    if d == 0 {
                        return MinCostFlowResult::Cost(total_cost);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod chap3_5_min_cost_flow_tests {
    use super::*;
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

    #[test]
    fn test_editorial() {
        let mut mcf = MinCostFlow::with_len(5);
        mcf.add_edge(0, 1, 10, 2);
        mcf.add_edge(0, 2, 2, 4);
        mcf.add_edge(1, 2, 6, 6);
        mcf.add_edge(1, 3, 6, 2);
        mcf.add_edge(2, 4, 5, 2);
        mcf.add_edge(3, 2, 3, 3);
        mcf.add_edge(3, 4, 8, 6);

        println!("Before:\n{:?}", dbg::Tabular(&mcf.pretty()));

        let result = mcf.run(0, 4, 9);
        assert_eq!(
            result,
            MinCostFlowResult::Cost(2 * 7 + 4 * 2 + 6 * 1 + 2 * 6 + 3 * 2 + 2 * 5 + 6 * 4)
        );

        println!("After:\n{:?}", dbg::Tabular(&mcf.pretty()));

        println!("The cost of a minimum cost flow is {:?}.", &result);

        // Before:
        //  0 | [(1, 0/10, 2), (2, 0/2, 4)]
        //  1 | [(2, 0/6, 6), (3, 0/6, 2)]
        //  2 | [(4, 0/5, 2)]
        //  3 | [(2, 0/3, 3), (4, 0/8, 6)]
        //  4 | []
        //
        // After:
        //  0 | [(1, 7/10, 2), (2, 2/2, 4)]
        //  1 | [(2, 1/6, 6), (3, 6/6, 2)]
        //  2 | [(4, 5/5, 2)]
        //  3 | [(2, 2/3, 3), (4, 4/8, 6)]
        //  4 | []
        //
        // The cost of a minimum cost flow is Cost(80).
    }

    #[test]
    fn test_negative_cycle() {
        let mut mcf = MinCostFlow::with_len(3);
        mcf.add_edge(0, 1, 10, -2);
        mcf.add_edge(1, 2, 10, -2);
        mcf.add_edge(2, 0, 10, -2);

        println!("Before:\n{:?}", dbg::Tabular(&mcf.pretty()));

        let result = mcf.run(0, 2, 9);
        assert_eq!(result, MinCostFlowResult::NegativeCycle);

        println!("After:\n{:?}", dbg::Tabular(&mcf.pretty()));

        println!("The cost of a minimum cost flow is {:?}.", &result);
    }

    #[test]
    fn test_impossible() {
        let mut mcf = MinCostFlow::with_len(5);
        mcf.add_edge(0, 1, 10, 2);
        mcf.add_edge(0, 2, 2, 4);
        mcf.add_edge(1, 2, 6, 6);
        mcf.add_edge(1, 3, 6, 2);
        mcf.add_edge(2, 4, 5, 2);
        mcf.add_edge(3, 2, 3, 3);
        mcf.add_edge(3, 4, 8, 6);

        println!("Before:\n{:?}", dbg::Tabular(&mcf.pretty()));

        let result = mcf.run(0, 4, 100);
        assert_eq!(result, MinCostFlowResult::Impossible);

        println!("After:\n{:?}", dbg::Tabular(&mcf.pretty()));

        println!("The cost of a minimum cost flow is {:?}.", &result);
    }
}
