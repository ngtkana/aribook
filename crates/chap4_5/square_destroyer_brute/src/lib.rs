#![allow(clippy::many_single_char_names)]
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
// dbg {{{
#[allow(dead_code)]
mod dbg {
    #[macro_export]
    macro_rules! lg {
        () => {
            $crate::eprintln!("[{}:{}]", $crate::file!(), $crate::line!());
        };
        ($val:expr) => {
            match $val {
                tmp => {
                    eprintln!("[{}:{}] {} = {:?}",
                        file!(), line!(), stringify!($val), &tmp);
                    tmp
                }
            }
        };
        ($val:expr,) => { lg!($val) };
        ($($val:expr),+ $(,)?) => {
            ($(lg!($val)),+,)
        };
    }

    #[macro_export]
    macro_rules! msg {
            () => {
                compile_error!();
            };
            ($msg:expr) => {
                $crate::eprintln!("[{}:{}][{}]", $crate::file!(), $crate::line!(), $msg);
            };
            ($msg:expr, $val:expr) => {
                match $val {
                    tmp => {
                        eprintln!("[{}:{}][{}] {} = {:?}",
                            file!(), line!(), $msg, stringify!($val), &tmp);
                        tmp
                    }
                }
            };
            ($msg:expr, $val:expr,) => { msg!($msg, $val) };
            ($msg:expr, $($val:expr),+ $(,)?) => {
                ($(msg!($msg, $val)),+,)
            };
        }

    #[macro_export]
    macro_rules! tabular {
        ($val:expr) => {
            eprintln!(
                "[{}:{}] {}:\n{:?}",
                file!(),
                line!(),
                stringify!($val),
                crate::dbg::Tabular($val)
            );
        };
    }

    use std::fmt::{Debug, Formatter};

    #[derive(Clone)]
    pub struct Tabular<'a, T: Debug>(pub &'a [T]);
    impl<'a, T: Debug> Debug for Tabular<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            for i in 0..self.0.len() {
                writeln!(f, "{:2} | {:?}", i, &self.0[i])?;
            }
            Ok(())
        }
    }

    #[derive(Clone)]
    pub struct BooleanTable<'a>(pub &'a [Vec<bool>]);
    impl<'a> Debug for BooleanTable<'a> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            for i in 0..self.0.len() {
                writeln!(f, "{:2} | {:?}", i, BooleanSlice(&self.0[i]))?;
            }
            Ok(())
        }
    }

    #[derive(Clone)]
    pub struct BooleanSlice<'a>(pub &'a [bool]);
    impl<'a> Debug for BooleanSlice<'a> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(
                f,
                "{}",
                self.0
                    .iter()
                    .map(|&b| if b { "1 " } else { "0 " })
                    .collect::<String>()
            )?;
            Ok(())
        }
    }
}
// }}}
use ordtools::Ordtools;

#[derive(Debug, Clone)]
struct State {
    count: u32,
    score: u32,
    current_minimum: u32,
    now: usize,
    is_broken: Vec<bool>,
}

#[derive(Debug, Clone)]
struct Props {
    number_of_matches: usize,
    relevant_squares: Vec<Vec<usize>>,
    responsible_squares: Vec<Vec<usize>>,
    squares: Vec<Vec<bool>>,
    length_of_square: Vec<u32>,
}

pub fn solve(n: usize, eliminated_matches: &[u32]) -> (u32, u32) {
    let squares = preprocess(n, eliminated_matches);
    let length_of_square = squares
        .iter()
        .map(|v| v.iter().filter(|&&b| b).count() as u32)
        .collect();
    let h = squares.len();
    let w = squares[0].len();
    let mut state = State {
        count: 0,
        score: 0,
        current_minimum: std::u32::MAX,
        now: 0,
        is_broken: vec![false; h],
    };
    let props = Props {
        number_of_matches: w,
        responsible_squares: responsible_squares(&squares),
        relevant_squares: relevant_squares(&squares),
        squares,
        length_of_square,
    };
    state.dfs(&props);
    (state.current_minimum, state.count)
}

impl State {
    fn dfs(&mut self, props: &Props) {
        self.count += 1;
        if self.now == props.number_of_matches {
            self.current_minimum.change_min(self.score);
        } else if self.score + self.hstar(&props) < self.current_minimum {
            let breaking_squares = props.relevant_squares[self.now]
                .iter()
                .filter(|&&s| !self.is_broken[s])
                .collect::<Vec<_>>();
            let priority = {
                if props.responsible_squares[self.now]
                    .iter()
                    .any(|&s| !self.is_broken[s])
                {
                    Priority::Necessary
                } else if breaking_squares.is_empty() {
                    Priority::Forbidden
                } else {
                    Priority::EitherIsOk
                }
            };

            // 選ばないパターンです。
            match priority {
                Priority::Necessary => (),
                Priority::Forbidden | Priority::EitherIsOk => {
                    self.now += 1;
                    self.dfs(props);
                    self.now -= 1;
                }
            }
            // 選ぶパターンです。
            match priority {
                Priority::Forbidden => (),
                Priority::Necessary | Priority::EitherIsOk => {
                    self.now += 1;
                    self.score += 1;
                    breaking_squares.iter().for_each(|&&s| {
                        assert!(!self.is_broken[s]);
                        self.is_broken[s] = true;
                    });
                    self.dfs(&props);
                    self.score -= 1;
                    self.now -= 1;
                    breaking_squares.iter().for_each(|&&s| {
                        assert!(self.is_broken[s]);
                        self.is_broken[s] = false;
                    });
                }
            }
        }
    }

    fn hstar(&self, props: &Props) -> u32 {
        let mut alive = self
            .is_broken
            .iter()
            .enumerate()
            .filter_map(|(i, &b)| if b { Some(i) } else { None })
            .collect::<Vec<_>>();
        alive.sort_by_key(|&s| props.length_of_square[s]);
        let mut union_of_squares = vec![false; props.number_of_matches];
        let mut ans = 0;
        for v in alive.iter().map(|&s| &props.squares[s]) {
            if v.iter().zip(union_of_squares.iter()).any(|(&x, &y)| x && y) {
                continue;
            }
            ans += 1;
            for (x, y) in union_of_squares.iter_mut().zip(v.iter()) {
                *x |= y;
            }
        }
        ans
    }
}

#[derive(Debug, Clone, Copy)]
enum Priority {
    Necessary,
    EitherIsOk,
    Forbidden,
}

fn relevant_squares(squares: &[Vec<bool>]) -> Vec<Vec<usize>> {
    let mut relevant_squares = vec![vec![]; squares[0].len()];
    for (i, v) in squares.iter().enumerate() {
        for x in v
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| if x { Some(i) } else { None })
        {
            relevant_squares[x].push(i);
        }
    }
    relevant_squares
}

fn responsible_squares(squares: &[Vec<bool>]) -> Vec<Vec<usize>> {
    let mut responsible_squares = vec![vec![]; squares[0].len()];
    for (i, v) in squares.iter().enumerate() {
        responsible_squares[v
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, &x)| if x { Some(i) } else { None })
            .unwrap()]
        .push(i);
    }
    responsible_squares
}

fn preprocess(n: usize, eliminated_matches: &[u32]) -> Vec<Vec<bool>> {
    // All squares.
    let squares = match_square_table(n);

    // Eliminate squares.
    let mut squares = squares
        .iter()
        .filter(|v| eliminated_matches.iter().all(|&x| !v[x as usize]))
        .cloned()
        .collect::<Vec<_>>();

    // Eliminate matches.
    let mut is_essential = vec![false; 2 * n * (n + 1)];
    for v in &squares {
        for (x, y) in is_essential.iter_mut().zip(v.iter()) {
            *x |= y;
        }
    }
    let non_necessary_matches = is_essential
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| if b { None } else { Some(i) })
        .collect::<Vec<_>>();
    for v in squares.iter_mut() {
        for &x in non_necessary_matches.iter().rev() {
            v.remove(x);
        }
    }

    squares
}

pub fn match_square_table(n: usize) -> Vec<Vec<bool>> {
    let mut ans = Vec::new();
    for d in 1..=n {
        for i_start in 0..=n - d {
            for j_start in 0..=n - d {
                let x = i_start * (2 * n + 1) + j_start;
                let y = x + n;
                let mut row = vec![false; 2 * n * (n + 1)];
                for k in (0..d)
                    .map(|i| x + i)
                    .chain((0..d).map(|i| x + d * (2 * n + 1) + i))
                    .chain((0..d).map(|j| y + j * (2 * n + 1)))
                    .chain((0..d).map(|j| y + j * (2 * n + 1) + d))
                {
                    row[k] = true;
                }
                ans.push(row);
            }
        }
    }
    ans
}

#[cfg(test)]
mod chap4_5_square_destroyer_brute_tests {
    use super::*;
    #[test]
    fn test_match_square_table_1() {
        let result = match_square_table(1)
            .iter()
            .map(|v| encode(&v))
            .collect::<Vec<_>>();
        let expected = vec!["1111".to_owned()];
        assert_eq!(&result, &expected);
    }

    #[test]
    fn test_match_square_table_2() {
        let result = match_square_table(2)
            .iter()
            .map(|v| encode(&v))
            .collect::<Vec<_>>();
        let expected = [
            "101101000000",
            "010110100000",
            "000001011010",
            "000000101101",
            "111010010111",
        ]
        .iter()
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
        assert_eq!(&result, &expected);
    }

    #[test]
    fn test_sample_1() {
        let (ans, count) = solve(2, &[]);
        println!("Number of searched states: {}", count);
        assert_eq!(ans, 3);
    }

    #[test]
    fn test_sample_2() {
        let (ans, count) = solve(3, &[11, 16, 22]);
        println!("Number of searched states: {}", count);
        assert_eq!(ans, 3);
    }

    #[test]
    fn test_hand_1() {
        let (ans, count) = solve(3, &[8, 11, 12, 15]);
        println!("Number of searched states: {}", count);
        assert_eq!(ans, 4);
    }

    #[test]
    fn test_hand_2() {
        let (ans, count) = solve(3, &[]);
        println!("Number of searched states: {}", count);
        assert_eq!(ans, 6);
    }

    #[test]
    fn test_hand_3() {
        let (ans, count) = solve(4, &[2, 10, 25, 30, 31]);
        println!("Number of searched states: {}", count);
        assert_eq!(ans, 7);
    }

    #[test]
    fn test_hand_4() {
        let (ans, count) = solve(4, &[]);
        println!("Number of searched states: {}", count);
        assert_eq!(ans, 10);
    }

    #[test]
    fn test_hand_5() {
        let (ans, count) = solve(5, &[4, 10, 11, 12, 14, 15, 18, 20, 22, 25, 30, 35, 39]);
        println!("Number of searched states: {}", count);
        assert_eq!(ans, 9);
    }

    fn encode(seq: &[bool]) -> String {
        seq.iter()
            .map(|x| match x {
                false => '0',
                true => '1',
            })
            .collect()
    }
}
