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
pub struct FordFulkerson {
    pub graph: Vec<Vec<Edge>>,
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

    fn find_augumenting_path(&self, s: usize, t: usize) -> (u32, Vec<usize>) {
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
        let mut used = vec![false; self.graph.len()];
        let mut path = Vec::new();
        let f = dfs(s, t, std::u32::MAX, &mut used, &self.graph, &mut path);
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
            let (f, path) = self.find_augumenting_path(s, t);
            if f == 0 {
                return flow;
            }
            self.push_along_path(s, t, f, &path);
            flow += f;
        }
    }

    pub fn increase(&mut self, s: usize, t: usize, edge_id: usize) -> u32 {
        let (from, from_id) = self.edge_keys[edge_id];
        self.graph[from][from_id].cap += 1;
        let (f, path) = self.find_augumenting_path(s, t);
        if f == 0 {
            0
        } else {
            self.push_along_path(s, t, 1, &path);
            1
        }
    }

    pub fn decrease(&mut self, s: usize, t: usize, edge_id: usize) -> u32 {
        let (from, from_id) = self.edge_keys[edge_id];
        let (to, rev) = {
            let Edge { to, cap, rev } = &mut self.graph[from][from_id];
            if 0 < *cap {
                *cap -= 1;
                return 0;
            }
            (*to, *rev)
        };
        let (f0, path0) = self.find_augumenting_path(from, to);
        if f0 != 0 {
            self.push_along_path(from, to, 1, &path0);
            self.graph[to][rev].cap -= 1;
            0
        } else {
            let (f1, path1) = self.find_augumenting_path(from, s);
            let (f2, path2) = self.find_augumenting_path(t, to);
            assert_ne!(f1, 0);
            assert_ne!(f2, 0);
            self.push_along_path(from, s, 1, &path1);
            self.push_along_path(t, to, 1, &path2);
            self.graph[to][rev].cap -= 1;
            1
        }
    }
}

use proconio::input;

fn main() {
    input!(
        n: usize,
        m: usize,
        q: usize,
        s: usize,
        t: usize,
        uvc: [(usize, usize, u32); m]
    );
    let mut ff = FordFulkerson::with_len(n);
    uvc.iter().for_each(|&(u, v, c)| ff.add_edge(u, v, c));
    let mut flow = ff.run(s, t);
    println!("{}", flow);
    for _ in 0..q {
        input!(command: String);
        match command.as_ref() {
            "increase" => {
                input!(i: usize);
                flow += ff.increase(s, t, i);
                println!("{}", flow);
            }
            "decrease" => {
                input!(i: usize);
                flow -= ff.decrease(s, t, i);
                println!("{}", flow);
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod chap3_5_increase_decrease_capacity_tests {
    const BIN: &str = "chap3_5_increase_decrease_capacity";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn test_hand() {
        test_sample(
            r#"5 7 5 0 4
0 1 10
0 2 2
1 2 6
1 3 6
2 4 5
3 2 3
3 4 8
increase 3
increase 6
decrease 1
decrease 0
decrease 3
"#,
            r#"11
12
12
11
10
10
"#,
        );
    }
}
