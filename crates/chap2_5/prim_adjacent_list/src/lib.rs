use std::cmp::Reverse;

pub fn prim(g: &[Vec<(usize, u32)>]) -> u32 {
    let n = g.len();
    let mut ans = 0;
    let mut heap = std::collections::BinaryHeap::from(vec![(Reverse(0), 0)]);
    let mut used = vec![false; n];
    while let Some((Reverse(dx), x)) = heap.pop() {
        if used[x] {
            continue;
        }
        ans += dx;
        used[x] = true;
        for (y, w) in g[x].iter().copied().filter(|&(y, _)| !used[y]) {
            heap.push((Reverse(w), y));
        }
    }
    ans
}

#[cfg(test)]
mod chap2_5_prim_adjacent_list_tests {
    use super::*;

    #[test]
    fn test_wikipedia() {
        let g = [
            vec![(1, 7), (3, 5)],
            vec![(0, 7), (2, 8), (3, 9), (4, 7)],
            vec![(1, 8), (4, 5)],
            vec![(0, 5), (1, 9), (4, 15), (5, 6)],
            vec![(1, 7), (2, 5), (3, 15), (5, 8), (6, 9)],
            vec![(3, 6), (4, 8), (6, 11)],
            vec![(4, 9), (5, 11)],
        ];
        assert_eq!(prim(&g), 39);
    }
}
