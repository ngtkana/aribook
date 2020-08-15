#![allow(clippy::many_single_char_names)]
const SQRT: usize = 3;
const SIZE: usize = 9;

fn available(grid: [[Option<u8>; SIZE]; SIZE], i_now: usize, j_now: usize) -> Option<NonemptyVec> {
    let mut ckd = [false; SIZE];
    let i_block = i_now / 3 * 3;
    let j_block = j_now / 3 * 3;
    for x in (i_block..i_block + SQRT)
        .map(|i| (j_block..j_block + SQRT).map(move |j| (i, j)))
        .flatten()
        .chain((0..SIZE).map(|j| (i_now, j)))
        .chain((0..SIZE).map(|i| (i, j_now)))
        .filter_map(|(i, j)| grid[i][j])
    {
        ckd[x as usize - 1] = true;
    }
    NonemptyVec::from_vec(
        (0..SIZE)
            .filter(|&i| !ckd[i])
            .map(|i| i as u8 + 1)
            .collect(),
    )
}

#[derive(Debug, Clone)]
struct NonemptyVec(Vec<u8>);

impl NonemptyVec {
    pub fn from_vec(vec: Vec<u8>) -> Option<Self> {
        if vec.is_empty() {
            None
        } else {
            Some(Self(vec))
        }
    }

    pub fn into_inner(self) -> Vec<u8> {
        let Self(inner) = self;
        inner
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone)]
struct State {
    grid: [[Option<u8>; SIZE]; SIZE],
    count: usize,
}

#[derive(Debug, Clone)]
enum EmptyCellOrSolution {
    EmptyCell(EmptyCell),
    Solution(Solution),
}

#[derive(Debug, Clone)]
struct EmptyCell {
    i: usize,
    j: usize,
    available: NonemptyVec,
}

impl State {
    pub fn emptycell_or_solution(&self) -> Option<EmptyCellOrSolution> {
        let mut ans = EmptyCellOrSolution::Solution(Solution {
            grid: [[255; SIZE]; SIZE],
        });
        for (i, j, &cell) in self
            .grid
            .iter()
            .enumerate()
            .map(|(i, v)| v.iter().enumerate().map(move |(j, x)| (i, j, x)))
            .flatten()
        {
            match &mut ans {
                EmptyCellOrSolution::EmptyCell(previous_empty_cell) => {
                    if cell.is_none() {
                        let available = available(self.grid, i, j)?;
                        if available.len() < previous_empty_cell.available.len() {
                            ans = EmptyCellOrSolution::EmptyCell(EmptyCell { i, j, available });
                        }
                    }
                }
                EmptyCellOrSolution::Solution(solution) => {
                    if let Some(x) = cell {
                        solution.grid[i][j] = x;
                    } else {
                        let available = available(self.grid, i, j)?;
                        ans = EmptyCellOrSolution::EmptyCell(EmptyCell { i, j, available });
                    }
                }
            }
        }
        Some(ans)
    }
}

#[derive(Debug, Clone)]
pub struct Solution {
    grid: [[u8; SIZE]; SIZE],
}

fn dfs(state: &mut State) -> Option<Solution> {
    state.count += 1;
    match state.emptycell_or_solution()? {
        EmptyCellOrSolution::Solution(solution) => Some(solution),
        EmptyCellOrSolution::EmptyCell(EmptyCell { i, j, available }) => {
            for x in available.into_inner() {
                state.grid[i][j] = Some(x);
                let solution = dfs(state);
                if solution.is_some() {
                    return solution;
                }
            }
            state.grid[i][j] = None;
            None
        }
    }
}

pub fn input_from_str(src: &str) -> [[Option<u8>; SIZE]; SIZE] {
    let mut s = src.split_whitespace();
    let mut grid = [[None; SIZE]; SIZE];
    for target_row in grid.iter_mut() {
        let source_row = s
            .next()
            .expect("Reached unexpected end of the input")
            .as_bytes();
        assert_eq!(source_row.len(), SIZE, "Invalid length of a row");
        for (src, tar) in source_row.iter().zip(target_row.iter_mut()) {
            *tar = match src {
                b'0' => None,
                c @ b'1'..=b'9' => Some(c - b'0'),
                _ => panic!("Received a non-digit cell"),
            };
        }
    }
    grid
}

pub fn output_from_str(src: &str) -> [[u8; SIZE]; SIZE] {
    let mut s = src.split_whitespace();
    let mut grid = [[255; SIZE]; SIZE];
    for target_row in grid.iter_mut() {
        let source_row = s
            .next()
            .expect("Reached unexpected end of the input")
            .as_bytes();
        assert_eq!(source_row.len(), SIZE, "Invalid length of a row");
        for (src, tar) in source_row.iter().zip(target_row.iter_mut()) {
            *tar = match src {
                c @ b'1'..=b'9' => c - b'0',
                _ => panic!("Received a non-digit cell"),
            };
        }
    }
    grid
}

pub fn solve(src: [[Option<u8>; SIZE]; SIZE]) -> (Option<[[u8; SIZE]; SIZE]>, usize) {
    let mut state = State {
        grid: src,
        count: 0,
    };

    (dfs(&mut state).map(|solution| solution.grid), state.count)
}

#[cfg(test)]
mod chap4_5_sudoku_priority_tests {
    use super::*;

    #[test]
    fn test_level_7() {
        let input_str = "000000520
            080400000
            030009000
            501000600
            200700000
            000300000
            600010000
            359682714
            128574936
            ";
        let expected_str = "416837529
            982465371
            735129468
            571298643
            293746185
            864351297
            647913852
            359682714
            128574936
            ";
        let input = input_from_str(&input_str);
        let expected = output_from_str(&expected_str);
        let (result, count) = solve(input);
        println!("Search count: {}", count);
        let result = result.unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_level_8() {
        let input_str = "000000520
            080400000
            030009000
            501000600
            200700000
            000300000
            600010000
            000000704
            128574936
            ";
        let expected_str = "416837529
            982465371
            735129468
            571298643
            293746185
            864351297
            647913852
            359682714
            128574936
            ";
        let input = input_from_str(&input_str);
        let expected = output_from_str(&expected_str);
        let (result, count) = solve(input);
        println!("Search count: {}", count);
        let result = result.unwrap();
        assert_eq!(result, expected);
    }
}
