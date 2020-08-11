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
use proconio::input;

#[allow(clippy::many_single_char_names)]
fn main() {
    input!(h: usize, w: usize, a: [[u8; w]; h]);
    let a = a
        .iter()
        .map(|v| v.iter().map(|&x| x == 1).collect::<Vec<bool>>())
        .collect::<Vec<_>>();

    if let Some(flip) = (0..1 << h)
        .filter_map(|bs| {
            let mut flip = vec![vec![false; w]; h];
            for j in (0..w).filter(|j| bs >> j & 1 == 1) {
                flip[0][j] = true;
            }
            let get = |i: usize, j: usize, flip: &mut [Vec<bool>]| -> bool {
                a[i][j] ^ {
                    let i = i as i32;
                    let j = j as i32;
                    [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)]
                        .iter()
                        .filter(|&(di, dj)| {
                            let ni = i + di;
                            let nj = j + dj;
                            0 <= ni
                                && ni < h as i32
                                && 0 <= nj
                                && nj < w as i32
                                && (flip[ni as usize][nj as usize])
                        })
                        .count()
                        % 2
                        == 1
                }
            };
            for i in 1..h {
                for j in 0..w {
                    let b = get(i - 1, j, &mut flip);
                    if b {
                        flip[i][j] = true;
                    }
                }
            }
            if (0..w).all(|j| !get(h - 1, j, &mut flip)) {
                Some(flip)
            } else {
                None
            }
        })
        .min_by_key(|flip| flip.iter().flatten().filter(|&&b| b).count())
    {
        for v in flip {
            println!(
                "{}",
                v.iter().map(|&b| (if b { 1 } else { 0 }).to_string()).fold(
                    String::new(),
                    |acc, s| if acc.is_empty() {
                        s
                    } else {
                        acc + " " + s.as_ref()
                    }
                )
            );
        }
    } else {
        println!("IMPOSSIBLE");
    }
}

#[cfg(test)]
mod chap3_2_fliptile_tests {
    const BIN: &str = "chap3_2_fliptile";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"4 4
1 0 0 1
0 1 1 0
0 1 1 0
1 0 0 1
"#,
            r#"0 0 0 0
1 0 0 1
1 0 0 1
0 0 0 0
"#,
        );
    }
}
