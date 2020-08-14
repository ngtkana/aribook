#![allow(clippy::many_single_char_names)]
const GRID_FOUR_DIRS_I64: [(i64, i64); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
use chap4_1_gauss_jordan::solve_non_singular_linear_eq_gauss_jordan;
use std::ops::RangeBounds;

fn grid_adj(
    i_range: impl RangeBounds<i64>,
    j_range: impl RangeBounds<i64>,
    i: usize,
    j: usize,
    dirs: &[(i64, i64)],
) -> Vec<(usize, usize)> {
    dirs.iter()
        .filter_map(|(di, dj)| {
            let ni = i as i64 + di;
            let nj = j as i64 + dj;
            if i_range.contains(&ni) && j_range.contains(&nj) {
                Some((ni as usize, nj as usize))
            } else {
                None
            }
        })
        .collect()
}

use proconio::input;

fn main() {
    input!(h: usize, w: usize, grid: [proconio::marker::Chars; h]);
    let n = h * w;
    let encode = |i: usize, j: usize| i * w + j;
    let mut graph = vec![None; n];
    let mut queue = std::collections::VecDeque::from(vec![(0, 0)]);
    while let Some((i, j)) = queue.pop_front() {
        let adj = grid_adj(0..h as i64, 0..w as i64, i, j, &GRID_FOUR_DIRS_I64)
            .into_iter()
            .filter(|&(ni, nj)| grid[ni][nj] == '.')
            .collect::<Vec<_>>();
        graph[encode(i, j)] = Some(
            adj.iter()
                .map(|&(ni, nj)| encode(ni, nj))
                .collect::<Vec<_>>(),
        );
        for &(ni, nj) in &adj {
            if graph[encode(ni, nj)].is_none() {
                queue.push_back((ni, nj));
            }
        }
    }

    let mut a = vec![vec![0.0; n]; n];
    let mut b = vec![0.0; n];
    (0..n).for_each(|i| a[i][i] = 1.0);
    for ((v, y), adj) in a.iter_mut().zip(b.iter_mut()).zip(graph.iter()).take(n - 1) {
        if let Some(adj) = adj {
            assert!(!adj.is_empty());
            *y = 1.0;
            let c = -(adj.len() as f64).recip();
            for &j in adj {
                v[j] = c;
            }
        }
    }
    let ans = solve_non_singular_linear_eq_gauss_jordan(&a, &b).unwrap();
    println!("{:.4}", ans[0]);
}

#[cfg(test)]
mod chap4_1_random_walk_tests {
    const BIN: &str = "chap4_1_random_walk";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"10 10
..######.#
......#..#
.#.##.##.#
.#........
##.##.####
....#....#
.#######.#
....#.....
.####.####
....#.....
"#,
            "1678.0000\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"10 10
..........
..........
..........
..........
..........
..........
..........
..........
..........
..........
"#,
            "542.1005\n",
        );
    }

    #[test]
    fn sample3() {
        test_sample(
            r#"3 10
.#...#...#
.#.#.#.#.#
...#...#..
"#,
            "361.0000\n",
        );
    }
}
