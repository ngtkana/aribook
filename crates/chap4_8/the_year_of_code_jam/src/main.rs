#![allow(clippy::many_single_char_names)]
const GRID_FOUR_DIRS_I32: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
use chap3_5_dinic::Dinic;
use proconio::{input, marker::Chars};

// コンテストを諦めるための代償です。
fn cost(c: char) -> i32 {
    match c {
        '?' => 4,
        '#' => std::i32::MAX / 2,
        '.' => 0,
        _ => panic!(),
    }
}

fn main() {
    input!(h: usize, w: usize, grid: [Chars; h]);
    // すべてのコンテストをノーコストで開催した場合の嬉しさです。
    let full_happiness = grid
        .iter()
        .flatten()
        .filter(|&&c| c == '?' || c == '#')
        .count() as i32
        * 4;

    // そこから比較した損失を計算です。
    let encode = |i: usize, j: usize| -> usize { i * w + j };
    let s = h * w;
    let t = h * w + 1;
    let mut dinic = Dinic::with_len(h * w + 2);
    for (i, gi) in grid.iter().enumerate() {
        for (j, &c) in gi.iter().enumerate() {
            let p = (i + j) % 2;
            match p {
                0 => {
                    dinic.add_edge(s, encode(i, j), cost(c));
                    for (ni, nj) in GRID_FOUR_DIRS_I32
                        .iter()
                        .map(|(di, dj)| (i as i32 + di, j as i32 + dj))
                        .filter(|&(ni, nj)| {
                            (0..h as i32).contains(&ni) && (0..w as i32).contains(&nj)
                        })
                        .map(|(ni, nj)| (ni as usize, nj as usize))
                    {
                        dinic.add_edge(encode(i, j), encode(ni, nj), 2);
                    }
                }
                1 => dinic.add_edge(encode(i, j), t, cost(c)),
                _ => panic!(),
            }
        }
    }
    let loss = dinic.run(s, t);
    let ans = full_happiness - loss;
    println!("{}", ans);
}

#[cfg(test)]
mod chap4_8_the_year_of_code_jam_tests {
    const BIN: &str = "chap4_8_the_year_of_code_jam";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3 3
.?.
.?.
.#.
"#,
            "8\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"5 8
.#...##.
.##..?..
.###.#.#
??#..?..
###?#...
"#,
            "42\n",
        );
    }
}
