#![allow(clippy::many_single_char_names)]
use chap4_3_sparse_table_argmin::SparseTableArgmin;

pub struct LcaRmq {
    pub order: Vec<usize>,
    pub spt: SparseTableArgmin<u32>,
    pub start: Vec<usize>,
}

impl LcaRmq {
    pub fn with_graph(root: usize, graph: &[Vec<usize>]) -> Self {
        fn dfs(
            x: usize,
            p: usize,
            d: u32,
            order: &mut Vec<usize>,
            depth: &mut Vec<u32>,
            graph: &[Vec<usize>],
        ) {
            for &y in graph[x].iter().filter(|&&y| y != p) {
                order.push(x);
                depth.push(d);
                dfs(y, x, d + 1, order, depth, graph);
            }
            order.push(x);
            depth.push(d);
        }
        let mut order = Vec::new();
        let mut depth = Vec::new();
        dfs(root, root, 0, &mut order, &mut depth, &graph);

        let mut start = vec![0; graph.len()];
        for (i, &x) in order.iter().enumerate() {
            start[x] = i;
        }

        Self {
            order,
            spt: SparseTableArgmin::from_vec(depth),
            start,
        }
    }

    pub fn query(&self, u: usize, v: usize) -> usize {
        assert!((0..self.start.len()).contains(&u));
        assert!((0..self.start.len()).contains(&v));
        let mut u = self.start[u];
        let mut v = self.start[v];
        if u > v {
            std::mem::swap(&mut u, &mut v);
        }
        self.order[self.spt.argmin(u..v + 1)]
    }
}

#[cfg(test)]
mod chap4_3_lca_rmq_tests {
    use super::*;

    #[test]
    fn test_editorial() {
        let g = [
            vec![1, 2],
            vec![0, 3, 4],
            vec![0, 5],
            vec![1],
            vec![1, 6, 7],
            vec![2],
            vec![4],
            vec![4],
        ];
        let lca = LcaRmq::with_graph(0, &g);
        assert_eq!(lca.query(3, 3), 3);
        assert_eq!(lca.query(3, 1), 1);
        assert_eq!(lca.query(6, 7), 4);
        assert_eq!(lca.query(5, 1), 0);
    }
}
