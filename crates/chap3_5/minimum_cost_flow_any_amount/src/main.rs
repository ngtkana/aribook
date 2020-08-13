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
use chap3_5_minimum_cost_flow::{MinCostFlow, ShortestPathResult};
use proconio::input;

fn main() {
    input!(
        n: usize,
        m: usize,
        s: usize,
        t: usize,
        uvcd: [(usize, usize, u32, i32); m]
    );
    let mut mcf = MinCostFlow::with_len(n);
    for (u, v, c, d) in uvcd {
        mcf.add_edge(u, v, c, d);
    }
    let ans = {
        let mut total_cost = 0;
        loop {
            match mcf.find_shortest_path_bellmann_ford(s, t) {
                ShortestPathResult::Unreachable => {
                    break total_cost;
                }
                ShortestPathResult::NegativeCycle => {
                    println!("Negative cycle");
                    std::process::exit(0);
                }
                ShortestPathResult::Finite { path, cost } => {
                    if 0 <= cost {
                        break total_cost;
                    }
                    let f = mcf.min_cap_flow_along_path(s, &path);
                    total_cost += f as i32 * cost;
                    mcf.push_along_path(s, f, &path);
                }
            }
        }
    };
    println!("{}", ans);
}

#[cfg(test)]
mod chap3_5_minimum_cost_flow_any_amount_tests {
    const BIN: &str = "chap3_5_minimum_cost_flow_any_amount";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"6 7 0 5
0 1 10 1
1 2 10 2
0 3 10 -10
1 4 10 -10
2 5 5 -10
3 4 10 4
4 5 10 8
"#,
            "-40\n",
        );
    }
}
