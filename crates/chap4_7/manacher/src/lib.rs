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

pub fn manacher(a: &[u8]) -> Vec<usize> {
    let mut i = 0;
    let mut j = 0;
    let mut manacher = vec![0; a.len()];
    while i < a.len() {
        j = (j..)
            .find(|&j| {
                a.get(i + j)
                    .and_then(|&y| i.checked_sub(j).map(|i_minus_j| a[i_minus_j] != y))
                    .unwrap_or(true)
            })
            .unwrap();
        manacher[i] = j;
        let k = (1..)
            .find(|&k| {
                i.checked_sub(k)
                    .map(|i_minus_k| j <= k + manacher[i_minus_k])
                    .unwrap_or(true)
            })
            .unwrap();
        (1..k).for_each(|k| manacher[i + k] = manacher[i - k]);
        i += k;
        j -= k;
    }
    manacher
}

#[cfg(test)]
mod chap4_7_manacher_tests {
    use super::*;

    #[test]
    fn test_hand_1() {
        let input = "abracadabra";
        let expected = [1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1];
        let result = manacher(input.as_bytes());
        assert_eq!(result.as_slice(), &expected);
    }

    #[test]
    fn test_hand_2() {
        let input = "abaaababa";
        let expected = [1, 2, 1, 4, 1, 2, 3, 2, 1];
        let result = manacher(input.as_bytes());
        assert_eq!(result.as_slice(), &expected);
    }
}
