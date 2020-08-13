#![allow(clippy::many_single_char_names)]
#[derive(Debug, Clone)]
pub struct Edge {
    to: usize,
    cap: u32,
    rev: usize,
}

#[derive(Clone)]
pub struct PrettyEdge {
    to: usize,
    flow: u32,
    cap: u32,
}

impl std::fmt::Debug for PrettyEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}/{})", self.to, self.flow, self.cap)
    }
}

#[derive(Debug, Clone)]
pub struct FordFulkerson {
    graph: Vec<Vec<Edge>>,
    pub edge_keys: Vec<(usize, usize)>,
}

impl FordFulkerson {
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
                    .filter_map(|(j, &Edge { to, cap, rev })| {
                        if hash_set.get(&(i, j)).is_some() {
                            let revcap = self.graph[to][rev].cap;
                            Some(PrettyEdge {
                                to,
                                flow: revcap,
                                cap: cap + revcap,
                            })
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect()
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: u32) {
        let from_len = self.graph[from].len();
        let to_len = self.graph[to].len();
        self.edge_keys.push((from, from_len));
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

    pub fn max_flow(&mut self, s: usize, t: usize) -> u32 {
        fn dfs(
            x: usize,
            t: usize,
            f: u32,
            used: &mut [bool],
            g: &[Vec<Edge>],
            path: &mut Vec<usize>,
        ) -> u32 {
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
            let f = dfs(s, t, std::u32::MAX, &mut used, &self.graph, &mut path);
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

#[cfg(test)]
mod chap3_5_ford_fulkerson_tests {
    use super::*;

    #[test]
    fn test_tutorial() {
        let mut ff = FordFulkerson::with_len(5);
        ff.add_edge(0, 1, 10);
        ff.add_edge(0, 2, 2);
        ff.add_edge(1, 2, 6);
        ff.add_edge(1, 3, 6);
        ff.add_edge(2, 4, 5);
        ff.add_edge(3, 2, 3);
        ff.add_edge(3, 4, 8);
        assert_eq!(ff.max_flow(0, 4), 11);
    }
}
