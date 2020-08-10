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

pub fn floyd_warshall(adj: &mut [Vec<i32>]) {
    let n = adj.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let x = adj[i][k].saturating_add(adj[k][j]);
                adj[i][j].change_min(x);
            }
        }
    }
}

#[cfg(test)]
mod chap2_5_floyd_warshall_tests {
    use super::*;

    #[test]
    fn test_wikipedia() {
        const INF: i32 = std::i32::MAX;
        let mut adj = [
            vec![0, INF, -2, INF],
            vec![4, 0, 3, INF],
            vec![INF, INF, 0, 2],
            vec![INF, -1, INF, 0],
        ];
        floyd_warshall(&mut adj);
        assert_eq!(
            adj,
            [
                vec![0, -1, -2, 0],
                vec![4, 0, 2, 4],
                vec![5, 1, 0, 2],
                vec![3, -1, 1, 0],
            ]
        );
    }
}
