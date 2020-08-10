use std::cmp::Reverse;

pub fn dijkstra_restoration(start: usize, g: &[Vec<(usize, u32)>]) -> Vec<(usize, u32)> {
    let n = g.len();
    let mut dist = vec![(start, std::u32::MAX); n];
    dist[start] = (start, 0);
    let mut queue = std::collections::BinaryHeap::from(vec![(Reverse(0), start)]);
    while let Some((Reverse(dx), x)) = queue.pop() {
        if dist[x].1 < dx {
            continue;
        }
        for &(y, w) in &g[x] {
            let ndy = dist[x].1 + w;
            if ndy < dist[y].1 {
                dist[y] = (x, ndy);
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
        assert_eq!(
            dijkstra_restoration(0, &g),
            vec![(0, 0), (2, 3), (0, 1), (1, 4)]
        );
    }
}
