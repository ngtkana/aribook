#![allow(clippy::many_single_char_names)]

#[derive(Debug, Clone)]
pub struct Scc {
    pub graph: Vec<Vec<usize>>,
    reverse_graph: Vec<Vec<usize>>,
}

impl Scc {
    pub fn with_len(len: usize) -> Self {
        Self {
            graph: vec![Vec::new(); len],
            reverse_graph: vec![Vec::new(); len],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.graph[from].push(to);
        self.reverse_graph[to].push(from);
    }

    pub fn run(&self) -> Vec<Vec<usize>> {
        let n = self.graph.len();

        fn dfs_forward(x: usize, used: &mut [bool], ord: &mut Vec<usize>, g: &[Vec<usize>]) {
            used[x] = true;
            for &y in &g[x] {
                if used[y] {
                    continue;
                }
                dfs_forward(y, used, ord, g);
            }
            ord.push(x);
        }
        let mut used = vec![false; n];
        let mut ord = Vec::new();
        dfs_forward(0, &mut used, &mut ord, &self.graph);

        fn dfs_backward(x: usize, used: &mut [bool], members: &mut Vec<usize>, g: &[Vec<usize>]) {
            used[x] = true;
            members.push(x);
            for &y in &g[x] {
                if used[y] {
                    continue;
                }
                dfs_backward(y, used, members, g);
            }
        }
        let mut used = vec![false; n];
        let mut ans = Vec::new();
        for i in 0..n {
            if used[i] {
                continue;
            }
            let mut members = Vec::new();
            dfs_backward(i, &mut used, &mut members, &self.reverse_graph);
            ans.push(members);
        }
        ans
    }

    pub fn convert_into_belongingness_table(cmp: Vec<Vec<usize>>) -> Vec<usize> {
        let len = cmp.iter().map(|v| v.len()).sum::<usize>();
        let mut ans = vec![0; len];
        for (i, v) in cmp.iter().enumerate() {
            v.iter().for_each(|&x| ans[x] = i);
        }
        ans
    }
}

#[cfg(test)]
mod chap4_3_scc_tests {
    use super::*;

    #[test]
    fn test_hand1() {
        let mut scc = Scc::with_len(3);
        scc.add_edge(0, 1);
        scc.add_edge(1, 0);
        scc.add_edge(0, 2);
        let expected = vec![vec![0, 1], vec![2]];
        assert_eq!(scc.run(), expected);
    }

    #[test]
    fn test_hand2() {
        let mut scc = Scc::with_len(4);
        scc.add_edge(0, 1);
        scc.add_edge(2, 1);
        scc.add_edge(2, 3);
        let expected = vec![vec![0], vec![1], vec![2], vec![3]];
        assert_eq!(scc.run(), expected);
    }
}
