#![allow(clippy::many_single_char_names)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Edge {
    pub to: usize,
    pub cap: u32,
    pub cost: i32,
    pub rev: usize,
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
pub struct ShortestPathResult {
    path: Vec<usize>,
    dist: Vec<u32>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MinCostFlowResult {
    Cost(u32),
    Impossible,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DistWithPredecessor {
    dist: u32,
    predecessor: Option<Predecessor>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Predecessor {
    from: usize,
    edge_id: usize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MinCostFlowDijkstra {
    pub graph: Vec<Vec<Edge>>,
    potential: Vec<u32>,
    pub edge_keys: Vec<(usize, usize)>,
}

impl MinCostFlowDijkstra {
    pub fn with_len(len: usize) -> Self {
        Self {
            graph: vec![Vec::new(); len],
            potential: vec![0; len],
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

    pub fn restore_vertex_sequence_of_path(&self, s: usize, path: &[usize]) -> Vec<usize> {
        let mut now = s;
        let mut ans = vec![s];
        for &i in path.iter().rev() {
            let &Edge { to, .. } = &self.graph[now][i];
            ans.push(to);
            now = to;
        }
        ans
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

    fn find_shortest_path_dijkstra(&self, s: usize, t: usize) -> Option<ShortestPathResult> {
        use std::cmp::Reverse;
        let mut heap = std::collections::BinaryHeap::from(vec![(Reverse(0), s)]);
        let mut dist = vec![std::u32::MAX; self.graph.len()];
        let mut predecessor = vec![None; self.graph.len()];
        dist[s] = 0;
        while let Some((Reverse(dx), x)) = heap.pop() {
            if dist[x] < dx {
                continue;
            }
            for (i, &Edge { to: y, cost, .. }) in
                self.graph[x].iter().enumerate().filter(|(_, e)| e.cap != 0)
            {
                let ndy =
                    (dx as i32 + cost + self.potential[x] as i32 - self.potential[y] as i32) as u32;
                if ndy < dist[y] {
                    dist[y] = ndy;
                    predecessor[y] = Some(Predecessor {
                        from: x,
                        edge_id: i,
                    });
                    heap.push((Reverse(ndy), y));
                }
            }
        }
        if dist[t] == std::u32::MAX {
            None
        } else {
            let mut path = Vec::new();
            let mut now = t;
            while now != s {
                if let Some(Predecessor { from, edge_id }) = predecessor[now] {
                    path.push(edge_id);
                    now = from;
                } else {
                    unreachable!()
                }
            }
            Some(ShortestPathResult { path, dist })
        }
    }

    fn update_potential(&mut self, dist: &[u32]) {
        self.potential
            .iter_mut()
            .zip(dist.iter())
            .for_each(|(x, y)| *x += y);
    }

    pub fn run(&mut self, s: usize, t: usize, mut d: u32) -> MinCostFlowResult {
        if d == 0 {
            MinCostFlowResult::Cost(0)
        } else {
            let mut total_cost = 0;
            loop {
                if let Some(ShortestPathResult { path, dist }) =
                    self.find_shortest_path_dijkstra(s, t)
                {
                    let f = self.min_cap_flow_along_path(s, &path).min(d);
                    d -= f;
                    total_cost += (self.potential[t] + dist[t]) * f;
                    self.push_along_path(s, f, &path);
                    self.update_potential(&dist);
                    if d == 0 {
                        break MinCostFlowResult::Cost(total_cost);
                    }
                } else {
                    break MinCostFlowResult::Impossible;
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
        let mut mcf = MinCostFlowDijkstra::with_len(5);
        mcf.add_edge(0, 1, 10, 2);
        mcf.add_edge(0, 2, 2, 4);
        mcf.add_edge(1, 2, 6, 6);
        mcf.add_edge(1, 3, 6, 2);
        mcf.add_edge(2, 4, 5, 2);
        mcf.add_edge(3, 2, 3, 3);
        mcf.add_edge(3, 4, 8, 6);

        println!("Before:\n{:?}", dbg::Tabular(&mcf.pretty()));

        let result = mcf.run(0, 4, 9);

        println!("After:\n{:?}", dbg::Tabular(&mcf.pretty()));

        assert_eq!(
            result,
            MinCostFlowResult::Cost(2 * 7 + 4 * 2 + 6 * 1 + 2 * 6 + 3 * 2 + 2 * 5 + 6 * 4)
        );

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
    fn test_impossible() {
        let mut mcf = MinCostFlowDijkstra::with_len(5);
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
