#![allow(clippy::many_single_char_names)]
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
use ordtools::Ordtools;
use proconio::{input, marker::Chars};

const KIND: usize = 4;

fn encode(c: char) -> usize {
    match c {
        'A' => 0,
        'G' => 1,
        'C' => 2,
        'T' => 3,
        _ => panic!("あのですね"),
    }
}

fn main() {
    input!(s: Chars, n: usize, forbidden: [Chars; n]);
    let s = s.iter().copied().map(encode).collect::<Vec<_>>();
    let forbidden = forbidden
        .iter()
        .map(|v| v.iter().copied().map(encode).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Prefix を全列挙です。
    let mut prefixes: Vec<Vec<usize>> = vec![vec![]];
    for f in &forbidden {
        prefixes.extend((1..=f.len()).map(|i| f[..i].iter().copied().collect::<Vec<_>>()));
    }
    prefixes.sort();
    prefixes.dedup();

    // 禁止文字列になっている prefix にチェックです。
    let mut is_forbidden = vec![false; prefixes.len()];
    for f in &forbidden {
        is_forbidden[prefixes.binary_search(f).unwrap()] = true;
    }

    // 遷移を前計算です。
    let mut next = vec![[0; KIND]; prefixes.len()];
    for (i, prefix) in prefixes.iter().enumerate() {
        for c in 0..KIND {
            let s = prefix
                .iter()
                .copied()
                .chain(std::iter::once(c))
                .collect::<Vec<_>>();
            for start in 0..s.len() {
                if let Ok(j) = prefixes.binary_search(&s[start..].to_vec()) {
                    next[i][c] = j;
                    break;
                }
            }
        }
    }

    // 本番行きます。
    let mut dp = vec![0; prefixes.len()];
    for &c in &s {
        let mut swp = vec![std::u32::MAX; prefixes.len()];
        for (i, &x) in dp
            .iter()
            .enumerate()
            .filter(|&(i, &x)| !is_forbidden[i] && x != std::u32::MAX)
        {
            for (d, &j) in next[i].iter().enumerate() {
                let y = if c == d { x } else { x + 1 };
                swp[j].change_min(y);
            }
        }
        dp.copy_from_slice(&swp);
    }

    let ans = dp
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| if is_forbidden[i] { None } else { Some(x) })
        .min()
        .unwrap();
    if ans == std::u32::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

#[cfg(test)]
mod chap4_7_dna_repair_tests {
    const BIN: &str = "chap4_7_dna_repair";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"AAAG
2
AAA
AAG
"#,
            "1\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"TGAATG
2
A
TG
"#,
            "4\n",
        );
    }

    #[test]
    fn sample3() {
        test_sample(
            r#"AGT
4
A
G
C
T
"#,
            "-1\n",
        );
    }
}
