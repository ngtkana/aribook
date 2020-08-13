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
use proconio::{input, marker::Usize1};

fn main() {
    input!(n: usize, a: usize, b: usize);
    let s = a + n + n + b;
    let t = s + 1;
    let mut ff = chap3_5_ford_fulkerson::FordFulkerson::with_len(t + 1);
    (0..a).for_each(|i| ff.add_edge(s, i, 1));
    (0..n).for_each(|i| ff.add_edge(a + i, a + n + i, 1));
    (0..b).for_each(|i| ff.add_edge(a + n + n + i, t, 1));
    for i in 0..n {
        input!(k: usize);
        for _ in 0..k {
            input!(x: Usize1);
            ff.add_edge(x, a + i, 1);
        }
        input!(k: usize);
        for _ in 0..k {
            input!(x: Usize1);
            ff.add_edge(a + n + i, a + n + n + x, 1);
        }
    }
    println!("{}", ff.max_flow(s, t));
}

#[cfg(test)]
mod chap3_5_dining_tests {
    const BIN: &str = "chap3_5_dining";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"4 3 3
2 1 2
2 2 3
2 1 3
2 1 3
2 1 3
2 1 2
2 1 2
1 3
"#,
            "3\n",
        );
    }
}
