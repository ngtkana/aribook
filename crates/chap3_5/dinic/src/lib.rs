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
pub struct Dinic {
    pub graph: Vec<Vec<Edge>>,
    pub edge_keys: Vec<(usize, usize)>,
}

#[derive(Debug, Clone)]
struct Env {
    level: Vec<u32>,
    iter: Vec<usize>,
}

impl Dinic {
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

    fn calc_level(&self, s: usize) -> Vec<u32> {
        let mut level = vec![std::u32::MAX; self.graph.len()];
        let mut queue = std::collections::VecDeque::from(vec![s]);
        level[s] = 0;
        while let Some(from) = queue.pop_front() {
            for &Edge { to, cap, .. } in &self.graph[from] {
                if cap != 0 && level[to] == std::u32::MAX {
                    queue.push_back(to);
                    level[to] = level[from] + 1;
                }
            }
        }
        level
    }

    fn find_augumenting_path_dinic(&self, s: usize, t: usize, env: &mut Env) -> (u32, Vec<usize>) {
        fn dfs(
            me: &Dinic,
            from: usize,
            t: usize,
            f: u32,
            env: &mut Env,
            path: &mut Vec<usize>,
        ) -> u32 {
            if from == t {
                return f;
            }
            while env.iter[from] != me.graph[from].len() {
                let Edge { to, cap, .. } = me.graph[from][env.iter[from]];
                if env.level[from] < env.level[to] {
                    let d = dfs(me, to, t, f.min(cap), env, path);
                    if d != 0 {
                        path.push(env.iter[from]);
                        return d;
                    }
                }
                env.iter[from] += 1;
            }
            0
        }
        let mut path = Vec::new();
        let f = dfs(&self, s, t, std::u32::MAX, env, &mut path);
        (f, path)
    }

    fn push_along_path(&mut self, s: usize, t: usize, f: u32, path: &[usize]) {
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
    }

    pub fn run(&mut self, s: usize, t: usize) -> u32 {
        let mut flow = 0;
        loop {
            let level = self.calc_level(s);
            if level[t] == std::u32::MAX {
                return flow;
            }
            let mut env = Env {
                level,
                iter: vec![0; self.graph.len()],
            };
            loop {
                let (f, path) = self.find_augumenting_path_dinic(s, t, &mut env);
                if f == 0 {
                    break;
                }
                self.push_along_path(s, t, f, &path);
                flow += f;
            }
        }
    }
}

#[cfg(test)]
mod chap3_5_dinic_tests {
    use super::*;

    #[test]
    fn test_tutorial() {
        let mut dinic = Dinic::with_len(5);
        dinic.add_edge(0, 1, 10);
        dinic.add_edge(0, 2, 2);
        dinic.add_edge(1, 2, 6);
        dinic.add_edge(1, 3, 6);
        dinic.add_edge(2, 4, 5);
        dinic.add_edge(3, 2, 3);
        dinic.add_edge(3, 4, 8);
        assert_eq!(dinic.run(0, 4), 11);
    }
}
