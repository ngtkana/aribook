#![allow(clippy::many_single_char_names)]

pub fn doubling(src: Vec<usize>) -> Vec<Vec<usize>> {
    let n = src.len();
    if n <= 1 {
        vec![src]
    } else {
        let ht = n.next_power_of_two().trailing_zeros();
        let mut tar = vec![src];
        let mut d = 1;
        for _ in 0..ht - 1 {
            let prv: &Vec<usize> = tar.last().unwrap();
            let mut row: Vec<usize> = prv.clone();
            for x in &mut row {
                *x = prv[*x];
            }
            tar.push(row);
            d *= 2;
        }
        assert!((n / 2..n).contains(&d));
        tar
    }
}

pub struct LcaDoubling {
    ancestor: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

impl LcaDoubling {
    pub fn with_graph(root: usize, graph: &[Vec<usize>]) -> Self {
        let mut parent = vec![root; graph.len()];
        let mut depth = vec![0; graph.len()];
        fn dfs(
            x: usize,
            p: usize,
            parent: &mut [usize],
            depth: &mut [usize],
            graph: &[Vec<usize>],
        ) {
            for &y in graph[x].iter().filter(|&&y| y != p) {
                depth[y] = depth[x] + 1;
                parent[y] = x;
                dfs(y, x, parent, depth, graph);
            }
        }
        dfs(root, root, &mut parent, &mut depth, &graph);

        Self {
            ancestor: doubling(parent),
            depth,
        }
    }

    pub fn query(&self, mut u: usize, mut v: usize) -> usize {
        if self.depth[u] > self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }
        let diff = self.depth[v] - self.depth[u];
        for row in self.ancestor.iter().enumerate().filter_map(|(i, row)| {
            if diff >> i & 1 == 1 {
                Some(row)
            } else {
                None
            }
        }) {
            v = row[v];
        }
        assert_eq!(self.depth[u], self.depth[v]);
        if u == v {
            u
        } else {
            for row in self.ancestor.iter().rev() {
                if row[u] != row[v] {
                    u = row[u];
                    v = row[v];
                }
            }
            assert_ne!(u, v);
            assert_eq!(self.ancestor[0][u], self.ancestor[0][v]);
            self.ancestor[0][u]
        }
    }
}

#[cfg(test)]
mod chap4_3_lca_doubling_tests {
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
        let lca = LcaDoubling::with_graph(0, &g);
        assert_eq!(lca.query(3, 3), 3);
        assert_eq!(lca.query(3, 1), 1);
        assert_eq!(lca.query(6, 7), 4);
        assert_eq!(lca.query(5, 1), 0);
    }
}
