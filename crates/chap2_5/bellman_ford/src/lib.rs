pub fn bellman_ford(
    start: usize,
    vertex_number: usize,
    edges: &[(usize, usize, i32)],
) -> Option<Vec<i32>> {
    let mut dist = vec![std::i32::MAX; vertex_number];
    dist[start] = 0;
    fn relax(edges: &[(usize, usize, i32)], dist: &mut [i32]) -> bool {
        let mut update = false;
        for &(u, v, w) in edges {
            let ndv = dist[u].saturating_add(w);
            if ndv < dist[v] {
                update = true;
                dist[v] = ndv;
            }
        }
        update
    }
    if (0..vertex_number)
        .map(|_| relax(&edges, &mut dist))
        .last()
        .unwrap()
    {
        None
    } else {
        Some(dist)
    }
}

#[cfg(test)]
mod chap2_5_bellman_ford_tests {
    use super::*;

    #[test]
    fn test_hand() {
        let edges = [(0, 1, 2), (0, 2, 1), (1, 2, -8), (1, 3, 1), (2, 3, 5)];
        assert_eq!(bellman_ford(0, 4, &edges), Some(vec![0, 2, -6, -1]));
    }

    // 0 -> 1 -> 2 -> 0 が non-negative cycle (重み 0 )
    #[test]
    fn test_hand_non_negative_cycle() {
        let edges = [(0, 1, 2), (2, 0, 6), (1, 2, -8), (1, 3, 1), (2, 3, 5)];
        assert_eq!(bellman_ford(0, 4, &edges), Some(vec![0, 2, -6, -1]));
    }

    // 0 -> 1 -> 2 -> 0 が negative cycle (重み -5 )
    #[test]
    fn test_hand_negative_cycle() {
        let edges = [(0, 1, 2), (2, 0, 1), (1, 2, -8), (1, 3, 1), (2, 3, 5)];
        assert_eq!(bellman_ford(0, 4, &edges), None);
    }
}
