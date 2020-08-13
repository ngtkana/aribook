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
use chap3_5_minimum_cost_flow_dijkstra::{Edge, MinCostFlowDijkstra, MinCostFlowResult};
use proconio::input;

fn main() {
    input!(
        n: usize,
        m: usize,
        xyb: [(i32, i32, u32); n],
        pqc: [(i32, i32, u32); m],
        e: [[i32; m]; n],
    );
    let s = n + m;
    let t = s + 1;
    let mut mcf = MinCostFlowDijkstra::with_len(t + 1);
    let mut total_flow = 0;
    let mut propsed_cost = 0;
    for (i, &(.., b)) in xyb.iter().enumerate() {
        mcf.add_edge(s, i, b, 0);
        total_flow += b;
    }
    for (j, &(.., c)) in pqc.iter().enumerate() {
        mcf.add_edge(n + j, t, c, 0);
    }
    for (i, &(x, y, ..)) in xyb.iter().enumerate() {
        for (j, &(p, q, ..)) in pqc.iter().enumerate() {
            let cost = (x - p).abs() + (y - q).abs() + 1;
            mcf.add_edge(i, n + j, std::u32::MAX, cost);
            propsed_cost += cost * e[i][j];
        }
    }
    match mcf.run(s, t, total_flow) {
        MinCostFlowResult::Cost(optimal_cost) => {
            if optimal_cost == propsed_cost as u32 {
                println!("OPTIMAL");
            } else {
                println!("SUBOPTIMAL");
                for v in &mcf.graph[..n] {
                    let mut ans = vec![None; m];
                    for &Edge { to, rev, .. } in v {
                        if (n..n + m).contains(&to) {
                            ans[to - n] = Some(mcf.graph[to][rev].cap);
                        }
                    }
                    println!(
                        "{}",
                        ans.iter()
                            .map(|x| x.unwrap().to_string())
                            .fold(String::new(), |acc, x| if acc.is_empty() {
                                x
                            } else {
                                acc + " " + x.as_ref()
                            })
                    );
                }
            }
        }
        MinCostFlowResult::Impossible => unreachable!(),
    }
}

#[cfg(test)]
mod chap3_5_evacuation_plan_flow_tests {
    const BIN: &str = "chap3_5_evacuation_plan_flow";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3 4
-3 3 5
-2 -2 6
2 2 5
-1 1 3
1 1 4
-2 -2 7
0 -1 3
3 1 1 0
0 0 6 0
0 3 0 2
"#,
            r#"SUBOPTIMAL
3 0 1 1
0 0 6 0
0 4 0 1
"#,
        );
    }
}
