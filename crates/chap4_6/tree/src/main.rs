#![allow(clippy::many_single_char_names)]
use proconio::{input, marker::Usize1};
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

fn solve(given_k: u32, graph: &[Vec<(usize, u32)>]) -> i64 {
    let n = graph.len();
    let mut states = States {
        graph,                       // 入力のグラフ
        is_centroid: vec![false; n], // 今見ているところよりも浅い重心
        subtree_size: vec![0; n],    // サイズ計算用
        given_k,                     // 距離がこれ以下のものを数え上げます。
    };
    let initial_vertex = 0;
    div_conq(initial_vertex, &mut states)
}

// 頂点 x が属する連結成分を重心分解する再帰関数です。
fn div_conq(x: usize, states: &mut States) -> i64 {
    // 重心探しパートです。
    initialize_size_buf(x, states);
    let mut l_states = LocalStates {
        total_size: states.subtree_size[x],
        found_centroid: None,
    };
    find_centroid(x, &mut l_states, states);
    let centroid = l_states.found_centroid.unwrap();
    states.is_centroid[centroid] = true;

    // 分割パートです。
    let mut ans_small = 0;
    for &(y, _) in &states.graph[x] {
        if !states.is_centroid[y] {
            ans_small += div_conq(y, states);
        }
    }

    // 計算パートです。
    let mut all_dists = vec![0];
    let mut ans_large = 0;
    for &(y, w) in &states.graph[x] {
        if !states.is_centroid[y] {
            let mut y_dists = enumerate_paths(y, w, states);
            y_dists.sort();
            ans_large -= count_pairs_of_sum_le_k(states.given_k, &y_dists);
            all_dists.append(&mut y_dists);
        }
    }
    all_dists.sort();
    ans_large += count_pairs_of_sum_le_k(states.given_k, &all_dists);
    states.is_centroid[centroid] = false;
    ans_small + ans_large
}

// x から到達可能なすべての頂点について、
// その subtree_size が x を根とする根付き木の部分木としてのサイズになるようにします。
fn initialize_size_buf(x: usize, states: &mut States) {
    fn dfs(x: usize, p: usize, states: &mut States) {
        let mut subtree_size_of_x = 1;
        for &(y, _) in &states.graph[x] {
            if y != p && !states.is_centroid[y] {
                dfs(y, x, states);
                subtree_size_of_x += states.subtree_size[y];
            }
        }
        states.subtree_size[x] = subtree_size_of_x;
    }
    dfs(x, x, states)
}

// x から到達可能な頂点のなす部分木の重心を探します。
//
// # Requirements
// - l_states: 初期状態です。
// - states: x を根として初期化されています。
fn find_centroid(x: usize, l_states: &mut LocalStates, states: &mut States) {
    fn dfs(x: usize, p: usize, l_states: &mut LocalStates, states: &mut States) {
        let mut maximum_branch_size_of_x = l_states.total_size - states.subtree_size[x];
        for &(y, _) in &states.graph[x] {
            if y != p && !states.is_centroid[y] {
                dfs(y, x, l_states, states);
                maximum_branch_size_of_x.change_max(states.subtree_size[y]);
            }
        }
        if 2 * maximum_branch_size_of_x <= l_states.total_size {
            l_states.found_centroid = Some(x);
        }
    }
    dfs(x, x, l_states, states)
}

// x から到達可能なすべての頂点について、
// その x からの距離を計算して詰めていきます。
fn enumerate_paths(x: usize, offset: u32, states: &States) -> Vec<u32> {
    fn dfs(x: usize, p: usize, offset: u32, path: &mut Vec<u32>, states: &States) {
        path.push(offset);
        for &(y, w) in &states.graph[x] {
            if y != p && !states.is_centroid[y] {
                dfs(y, x, offset + w, path, states);
            }
        }
    }
    let mut paths = Vec::new();
    dfs(x, x, offset, &mut paths, states);
    paths
}

// ソート列に対して、その合計が k 以下になるような相異なる
// 2 つの頂点の非順序対の個数を計算です。
fn count_pairs_of_sum_le_k(k: u32, sorted_seq: &[u32]) -> i64 {
    let mut j = sorted_seq.len();
    let mut ans = 0;
    for x in sorted_seq {
        while 0 != j && k < sorted_seq[j - 1] + x {
            j -= 1;
        }
        ans += j as i64;
    }
    ans -= sorted_seq.iter().filter(|&&x| x + x <= k).count() as i64;
    assert!(ans % 2 == 0);
    ans / 2
}

#[derive(Debug, Clone)]
struct States<'a> {
    graph: &'a [Vec<(usize, u32)>],
    subtree_size: Vec<usize>,
    is_centroid: Vec<bool>,
    given_k: u32,
}

#[derive(Debug, Clone)]
struct LocalStates {
    total_size: usize,
    found_centroid: Option<usize>,
}

fn main() {
    input!(n: usize, k: u32, uvw: [(Usize1, Usize1, u32); n - 1]);
    let mut g = vec![vec![]; n];
    for (u, v, w) in uvw {
        g[u].push((v, w));
        g[v].push((u, w));
    }
    let ans = solve(k, &g);
    println!("{}", ans);
}

#[cfg(test)]
mod chap4_6_tree_tests {
    const BIN: &str = "chap4_6_tree";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"5 4
1 2 3
1 3 1
1 4 2
3 5 1
"#,
            "8\n",
        );
    }
}
