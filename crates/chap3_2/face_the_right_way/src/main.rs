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
                    .map(|&b| if b { '#' } else { '.' })
                    .collect::<String>()
            )?;
            Ok(())
        }
    }
}
// }}}
use proconio::input;

fn main() {
    input!(n: usize, s: proconio::marker::Chars);
    let (operation_count, k) = (1..=n)
        .filter_map(|k| {
            let mut reverse_table = vec![false; n];
            let mut reversed = false;
            let mut operation_count = 0;
            for i in 0..n {
                if (s[i] == 'B') ^ reversed {
                    if n < i + k {
                        return None;
                    } else {
                        operation_count += 1;
                        reversed ^= true;
                        reverse_table[i] = true;
                    }
                }
                if k - 1 <= i && reverse_table[i - (k - 1)] {
                    reversed ^= true;
                }
            }
            Some((operation_count, k))
        })
        .min()
        .unwrap();
    println!("{} {}", operation_count, k);
}

#[cfg(test)]
mod chap3_2_face_the_right_way_tests {
    const BIN: &str = "chap3_2_face_the_right_way";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"7
BBFBFBB
"#,
            "3 3\n",
        );
    }
}
