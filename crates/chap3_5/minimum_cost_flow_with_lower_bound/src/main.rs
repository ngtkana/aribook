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

use chap3_5_minimum_cost_flow::{MinCostFlow, MinCostFlowResult};
use proconio::input;

fn main() {
    let large_cost = 10_000;
    input!(
        n: usize,
        m: usize,
        s: usize,
        t: usize,
        d: u32,
        uvlrc: [(usize, usize, u32, u32, i32); m]
    );
    let mut bonus = 0;
    let mut mcf = MinCostFlow::with_len(n);
    for (u, v, l, r, cost) in uvlrc {
        mcf.add_edge(u, v, r - l, cost);
        if l != 0 {
            mcf.add_edge(u, v, l, cost - large_cost);
            bonus += large_cost * l as i32;
        }
    }
    match mcf.run(s, t, d) {
        MinCostFlowResult::Cost(cost) => {
            println!("{}", cost as i32 + bonus);
        }
        MinCostFlowResult::Impossible => {
            println!("Impossible");
        }
        MinCostFlowResult::NegativeCycle => {
            println!("Negative cycle");
        }
    }
}

#[cfg(test)]
mod chap3_5_minimum_cost_flow_with_lower_bound_tests {
    const BIN: &str = "chap3_5_minimum_cost_flow_with_lower_bound";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn test_hand() {
        test_sample(
            r#"5 7 0 4 9
0 1 0 10 2
0 2 0 2 4
1 2 0 6 6
1 3 0 6 2
3 2 0 3 3
2 4 0 5 2
3 4 6 8 6
"#,
            "82\n",
        );
    }

    #[test]
    fn test_hand_impossible() {
        test_sample(
            r#"5 7 0 4 13
0 1 0 10 2
0 2 0 2 4
1 2 0 6 6
1 3 0 6 2
3 2 0 3 3
2 4 0 5 2
3 4 6 8 6
"#,
            "Impossible\n",
        );
    }
}
