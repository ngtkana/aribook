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

pub fn prim(adj: &[Vec<u32>]) -> u32 {
    let n = adj.len();
    let mut ans = 0;
    let mut used = vec![false; n];
    let mut min_cost = vec![std::u32::MAX; n];
    min_cost[0] = 0;
    while let Some(i) = (0..n).filter(|&i| !used[i]).min_by_key(|&i| min_cost[i]) {
        ans += min_cost[i];
        used[i] = true;
        (0..n).for_each(|j| min_cost[j].change_min(adj[i][j]));
    }
    ans
}

#[cfg(test)]
mod chap2_5_prim_adjacent_matrix_tests {
    use super::*;

    #[test]
    fn test_wikipedia() {
        const INF: u32 = std::u32::MAX;
        let adj = [
            vec![0, 7, INF, 5, INF, INF, INF],
            vec![7, 0, 8, 9, 7, INF, INF],
            vec![INF, 8, 0, INF, 5, INF, INF],
            vec![5, 9, INF, 0, 15, 6, INF],
            vec![INF, 7, 5, 15, 0, 8, 9],
            vec![INF, INF, INF, 6, 8, 0, 11],
            vec![INF, INF, INF, INF, 9, 11, 0],
        ];
        assert_eq!(prim(&adj), 39);
    }
}
