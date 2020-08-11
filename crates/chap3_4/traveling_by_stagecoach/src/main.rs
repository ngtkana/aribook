// dbg {{{
#[allow(dead_code)]
mod dbg {
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
use ordered_float::NotNan;
use ordtools::Ordtools;
use proconio::{input, marker::Usize1};

fn main() {
    input!(
        ticket_count: usize,
        n: usize,
        m: usize,
        start: Usize1,
        end: Usize1,
        t: [f64; ticket_count],
        uvw: [(Usize1, Usize1, f64); m]
    );
    let mut g = vec![vec![]; n];
    for (u, v, w) in uvw {
        g[u].push((v, w));
        g[v].push((u, w));
    }
    let mut dp = vec![vec![std::f64::INFINITY; n]; 1 << ticket_count];
    dp[(1 << ticket_count) - 1][start] = 0.0;
    for bs in (0..1 << ticket_count).rev() {
        for ticket in (0..ticket_count).filter(|&ticket| bs >> ticket & 1 == 1) {
            for (x, gx) in g.iter().enumerate() {
                let dpx = dp[bs][x];
                for &(y, w) in gx {
                    dp[bs ^ 1 << ticket][y].change_min(dpx + w / t[ticket]);
                }
            }
        }
    }
    if let Some(ans) = dp.iter().map(|v| NotNan::new(v[end]).unwrap()).min() {
        println!("{:.3}", ans);
    } else {
        println!("Impossible");
    }
}

#[cfg(test)]
mod chap3_4_traveling_by_stagecoach_tests {
    const BIN: &str = "chap3_4_traveling_by_stagecoach";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"2 4 4 2 1
3 1
1 3 3
1 4 2
2 3 3
2 4 5
"#,
            "3.667\n",
        );
    }
}
