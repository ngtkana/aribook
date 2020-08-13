#![allow(clippy::many_single_char_names)]
use std::ops::{Add, AddAssign, Sub, SubAssign};

macro_rules! impl_dinic_value_integer {
    ($($ty:ident,)*) => {$(
        impl DinicValue for $ty {
            fn zero() -> Self { 0 }
            fn max_value() -> Self { std::$ty::MAX }
        }
    )*}
}

impl_dinic_value_integer! {
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
}

pub trait DinicValue:
    Sized
    + std::fmt::Debug
    + Copy
    + Clone
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + std::cmp::Ord
{
    fn zero() -> Self;

    fn max_value() -> Self;
}

#[derive(Debug, Clone)]
pub struct Edge<Value: DinicValue> {
    to: usize,
    cap: Value,
    rev: usize,
}

#[derive(Clone)]
pub struct PrettyEdge<Value: DinicValue> {
    to: usize,
    flow: Value,
    cap: Value,
}

impl<Value: DinicValue> std::fmt::Debug for PrettyEdge<Value> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?}/{:?})", self.to, self.flow, self.cap)
    }
}

#[derive(Debug, Clone)]
pub struct Dinic<Value: DinicValue> {
    pub graph: Vec<Vec<Edge<Value>>>,
    pub edge_keys: Vec<(usize, usize)>,
}

#[derive(Debug, Clone)]
struct Env {
    level: Vec<u32>,
    iter: Vec<usize>,
}

impl<Value: DinicValue> Dinic<Value> {
    pub fn with_len(len: usize) -> Self {
        Self {
            graph: vec![Vec::new(); len],
            edge_keys: Vec::new(),
        }
    }

    pub fn pretty(&self) -> Vec<Vec<PrettyEdge<Value>>> {
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

    pub fn add_edge(&mut self, from: usize, to: usize, cap: Value) {
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
            cap: Value::zero(),
            rev: from_len,
        });
    }

    fn calc_level(&self, s: usize) -> Vec<u32> {
        let mut level = vec![std::u32::MAX; self.graph.len()];
        let mut queue = std::collections::VecDeque::from(vec![s]);
        level[s] = 0;
        while let Some(from) = queue.pop_front() {
            for &Edge { to, cap, .. } in &self.graph[from] {
                if cap != Value::zero() && level[to] == std::u32::MAX {
                    queue.push_back(to);
                    level[to] = level[from] + 1;
                }
            }
        }
        level
    }

    fn find_augumenting_path_dinic(
        &self,
        s: usize,
        t: usize,
        env: &mut Env,
    ) -> (Value, Vec<usize>) {
        fn dfs<Value: DinicValue>(
            me: &Dinic<Value>,
            from: usize,
            t: usize,
            f: Value,
            env: &mut Env,
            path: &mut Vec<usize>,
        ) -> Value {
            if from == t {
                return f;
            }
            let orig_iter = env.iter[from];
            while {
                let Edge { to, cap, .. } = me.graph[from][env.iter[from]];
                if env.level[from] < env.level[to] {
                    let d = dfs(me, to, t, f.min(cap), env, path);
                    if d != Value::zero() {
                        path.push(env.iter[from]);
                        return d;
                    }
                }
                env.iter[from] += 1;
                if env.iter[from] == me.graph[from].len() {
                    env.iter[from] = 0;
                }
                env.iter[from] != orig_iter
            } {}
            Value::zero()
        }
        let mut path = Vec::new();
        let f = dfs(&self, s, t, Value::max_value(), env, &mut path);
        (f, path)
    }

    fn push_along_path(&mut self, s: usize, t: usize, f: Value, path: &[usize]) {
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

    pub fn run(&mut self, s: usize, t: usize) -> Value {
        let mut flow = Value::zero();
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
                if f == Value::zero() {
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
        let mut dinic = Dinic::<u32>::with_len(5);
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
