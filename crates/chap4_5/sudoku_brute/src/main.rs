#![allow(clippy::many_single_char_names)]
use proconio::input;

const SQRT: usize = 3;
const SIZE: usize = 9;

fn available(seq: &[u8]) -> Vec<u8> {
    let mut ckd = [false; SIZE];
    for &x in seq {
        ckd[x as usize - 1] = true;
    }
    (0..SIZE)
        .filter(|&i| !ckd[i])
        .map(|i| i as u8 + 1)
        .collect()
}

#[derive(Debug, Clone)]
struct State {
    grid: [[Option<u8>; SIZE]; SIZE],
    empty_cells: Vec<(usize, usize)>,
    count: usize,
}

#[derive(Debug, Clone)]
enum EmptyCellOrSolution {
    EmptyCell((usize, usize)),
    Solution(Solution),
}

impl State {
    pub fn emptycell_or_solution(&self) -> EmptyCellOrSolution {
        let mut grid = [[0; SIZE]; SIZE];
        for (i, j, &x) in self
            .grid
            .iter()
            .enumerate()
            .map(|(i, v)| v.iter().enumerate().map(move |(j, x)| (i, j, x)))
            .flatten()
        {
            if let Some(x) = x {
                grid[i][j] = x;
            } else {
                return EmptyCellOrSolution::EmptyCell((i, j));
            }
        }
        EmptyCellOrSolution::Solution(Solution { grid })
    }
}

#[derive(Debug, Clone)]
struct Solution {
    grid: [[u8; SIZE]; SIZE],
}

fn solve(state: &mut State) -> Option<Solution> {
    state.count += 1;
    match state.emptycell_or_solution() {
        EmptyCellOrSolution::Solution(solution) => Some(solution),
        EmptyCellOrSolution::EmptyCell((i_now, j_now)) => {
            let i_block = i_now / 3 * 3;
            let j_block = j_now / 3 * 3;
            let available = available(
                &(i_block..i_block + SQRT)
                    .map(|i| (j_block..j_block + SQRT).map(move |j| (i, j)))
                    .flatten()
                    .chain((0..SIZE).map(|j| (i_now, j)))
                    .chain((0..SIZE).map(|i| (i, j_now)))
                    .filter_map(|(i, j)| state.grid[i][j])
                    .collect::<Vec<_>>(),
            );
            if available.is_empty() {
                return None;
            }
            for x in available {
                state.grid[i_now][j_now] = Some(x);
                state.empty_cells.pop();
                let solution = solve(state);
                state.empty_cells.push((i_now, j_now));
                if solution.is_some() {
                    return solution;
                }
            }
            state.grid[i_now][j_now] = None;
            None
        }
    }
}

fn main() {
    input!(char_grid: [proconio::marker::Chars; SIZE]);
    let mut grid = [[None; SIZE]; SIZE];
    let mut empty_cells = Vec::new();
    for i in 0..SIZE {
        for j in 0..SIZE {
            grid[i][j] = match char_grid[i][j] {
                '0' => {
                    empty_cells.push((i, j));
                    None
                }
                c @ '1'..='9' => Some(c as u8 - b'0'),
                _ => unreachable!(),
            };
        }
    }
    let mut state = State {
        grid,
        empty_cells,
        count: 0,
    };

    let solution = solve(&mut state).unwrap();
    for v in &solution.grid {
        println!(
            "{}",
            v.iter()
                .map(|x| x.to_string())
                .fold(String::new(), |acc, x| acc + x.as_ref())
        );
    }
}

#[cfg(test)]
mod chap4_5_sudoku_brute_tests {
    use super::*;

    const BIN: &str = "chap4_5_sudoku_brute";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    // Level 7 => Count = 635
    // Level 8 => Count = 6610
    // Level 9 => Count = 291406
    #[test]
    fn test_level_7() {
        test_sample(
            r#"000000520
080400000
030009000
501000600
200700000
000300000
600010000
359682714
128574936
"#,
            r#"416837529
982465371
735129468
571298643
293746185
864351297
647913852
359682714
128574936
"#,
        );
    }

    #[test]
    fn test_level_8() {
        test_sample(
            r#"000000520
080400000
030009000
501000600
200700000
000300000
600010000
000000704
128574936
"#,
            r#"416837529
982465371
735129468
571298643
293746185
864351297
647913852
359682714
128574936
"#,
        );
    }

    #[test]
    fn test_level_9() {
        test_sample(
            r#"000000520
080400000
030009000
501000600
200700000
000300000
600010000
000000704
000000030
"#,
            r#"416837529
982465371
735129468
571298643
293746185
864351297
647913852
359682714
128574936
"#,
        );
    }

    #[test]
    fn test_available() {
        let a = [4, 1, 5, 7, 4, 5, 6, 7, 5, 4, 3, 5, 5];
        let expected = [2, 8, 9];
        assert_eq!(available(&a), expected.to_vec());
    }
}
