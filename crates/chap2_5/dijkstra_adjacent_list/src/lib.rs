use std::cmp::Reverse;

pub fn dijkstra_adjacent_list(start: usize, g: &[Vec<(usize, u32)>]) -> Vec<u32> {
    let n = g.len();
    let mut dist = vec![std::u32::MAX; n];
    dist[start] = 0;
    let mut queue = std::collections::BinaryHeap::from(vec![(Reverse(0), start)]);
    while let Some((Reverse(dx), x)) = queue.pop() {
        if dist[x] < dx {
            continue;
        }
        for &(y, w) in &g[x] {
            let ndy = dist[x] + w;
            if ndy < dist[y] {
                dist[y] = ndy;
                queue.push((Reverse(ndy), y));
            }
        }
    }
    dist
}

#[cfg(test)]
mod chap2_5_dijkstra_adjacent_list_tests {
    use super::*;

    #[test]
    fn test_hand() {
        let g = [
            vec![(1, 4), (2, 1), (3, 9)],
            vec![(3, 1)],
            vec![(1, 2), (3, 4)],
            vec![],
        ];
        assert_eq!(dijkstra_adjacent_list(0, &g), vec![0, 3, 1, 4]);
    }
}
