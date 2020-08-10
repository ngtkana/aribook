// ordtools {{{
#[allow(dead_code)]
mod ordtools {
    pub trait Ordtools: PartialOrd + Sized {
        fn change_min(&mut self, mut rhs: Self) {
            if self > &mut rhs {
                *self = rhs;
            }
        }

        fn change_max(&mut self, mut rhs: Self) {
            if self < &mut rhs {
                *self = rhs;
            }
        }
    }

    impl<T: PartialOrd + Sized> Ordtools for T {}
}
// }}}
use ordtools::Ordtools;

pub fn dijkstra_adjacent_matrix(start: usize, adj: &[Vec<u32>]) -> Vec<u32> {
    let n = adj.len();
    let mut dist = vec![std::u32::MAX; n];
    let mut used = vec![false; n];
    dist[start] = 0;
    while let Some(x) = (0..n).filter(|&i| !used[i]).min_by_key(|&i| dist[i]) {
        used[x] = true;
        for y in 0..n {
            let ndy = dist[x] + adj[x][y];
            dist[y].change_min(ndy);
        }
    }
    dist
}

#[cfg(test)]
mod chap2_5_dijkstra_adjacent_matrix_tests {
    use super::*;

    #[test]
    fn hand() {
        let adj = [
            vec![0, 4, 1, 9],
            vec![4, 0, 2, 1],
            vec![1, 2, 0, 4],
            vec![9, 1, 4, 0],
        ];
        assert_eq!(dijkstra_adjacent_matrix(0, &adj), vec![0, 3, 1, 4]);
    }
}
