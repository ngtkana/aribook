#![allow(clippy::many_single_char_names)]
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
use proconio::input;
const GRID_FOUR_DIRS_I32: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn main() {
    input!(h: usize, w: usize, grid: [proconio::marker::Chars; h]);
    let a = grid.iter().flatten().filter(|&&c| c == 'D').count();
    let b = grid.iter().flatten().filter(|&&c| c == '.').count();
    let mut edges_by_dist = vec![vec![]; b + 1];
    let mut door_id = 0;
    for door_i in 0..h {
        for door_j in 0..w {
            if grid[door_i][door_j] != 'D' {
                continue;
            }
            let mut dist = vec![vec![std::u32::MAX; w]; h];
            dist[door_i][door_j] = 0;
            let mut queue = std::collections::VecDeque::from(vec![(door_i, door_j)]);
            while let Some((i, j)) = queue.pop_front() {
                for (di, dj) in GRID_FOUR_DIRS_I32.iter() {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if (0..h as i32).contains(&ni) && (0..w as i32).contains(&nj) {
                        let ni = ni as usize;
                        let nj = nj as usize;
                        if dist[ni][nj] == std::u32::MAX && grid[ni][nj] == '.' {
                            dist[ni][nj] = dist[i][j] + 1;
                            queue.push_back((ni, nj));
                        }
                    }
                }
            }
            for (person_id, d) in dist
                .iter()
                .flatten()
                .zip(grid.iter().flatten())
                .filter_map(|(&dist, &c)| if c == '.' { Some(dist) } else { None })
                .enumerate()
                .filter(|&(_, d)| d != std::u32::MAX)
            {
                edges_by_dist[d as usize].push((door_id, person_id));
            }
            door_id += 1;
        }
    }

    let s = a + b;
    let t = a + b + 1;
    let mut ff = chap3_5_ford_fulkerson::FordFulkerson::with_len(a + b + 2);
    (0..b).for_each(|j| ff.add_edge(a + j, t, 1));
    let mut flow = 0;
    for (ans, edges) in edges_by_dist.iter().enumerate().skip(1) {
        (0..a).for_each(|i| ff.add_edge(s, i, 1));
        for &(door_id, person_id) in edges {
            ff.add_edge(door_id, a + person_id, 1);
        }
        flow += ff.max_flow(s, t);
        if flow == b as u32 {
            println!("{}", ans);
            std::process::exit(0);
        }
    }
    println!("impossible");
}

#[cfg(test)]
mod chap3_5_evacuation_fast_tests {
    const BIN: &str = "chap3_5_evacuation_fast";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"5 5
XXDXX
X...X
D...X
X...D
XXXXX
"#,
            "3\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"5 12
XXXXXXXXXXXX
X..........D
X.XXXXXXXXXX
X..........X
XXXXXXXXXXXX
"#,
            "21\n",
        );
    }

    #[test]
    fn sample3() {
        test_sample(
            r#"5 5
XDXXX
X.X.D
XX.XX
D.X.X
XXXDX
"#,
            "impossible\n",
        );
    }
}
