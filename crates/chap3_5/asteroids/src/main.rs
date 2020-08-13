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
use proconio::{input, marker::Usize1};

fn main() {
    input!(n: usize, k: usize, xy: [(Usize1, Usize1); k]);
    let mut dinic = chap3_5_dinic::Dinic::with_len(2 * k + 2);
    let s = 2 * k;
    let t = 2 * k + 1;
    (0..n).for_each(|i| dinic.add_edge(s, i, 1));
    (0..n).for_each(|i| dinic.add_edge(i + n, t, 1));
    xy.iter().for_each(|&(u, v)| dinic.add_edge(u, n + v, 1));
    println!("{}", dinic.run(s, t));
}

#[cfg(test)]
mod chap3_5_asteroids_tests {
    const BIN: &str = "chap3_5_asteroids";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3 4
1 1
1 3
2 2
3 2
"#,
            "2\n",
        );
    }
}
