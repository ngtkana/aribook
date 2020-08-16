#![allow(clippy::many_single_char_names)]
use proconio::{input, marker::Chars};
use std::{collections::HashSet, num::Wrapping};

const BASE_I: Wrapping<u64> = Wrapping(1_000_000_007);
const BASE_J: Wrapping<u64> = Wrapping(9_973);

fn pow(p: usize, base: Wrapping<u64>) -> Wrapping<u64> {
    match p {
        0 => Wrapping(1),
        1 => base,
        p => pow(p / 2, base) * pow((p + 1) / 2, base),
    }
}

fn hash_seq(a: &[Wrapping<u64>], base: Wrapping<u64>) -> Wrapping<u64> {
    let mut ans = Wrapping(0u64);
    for &x in a {
        ans *= base;
        ans += x;
    }
    ans
}

fn transpose(a: &[Vec<Wrapping<u64>>]) -> Vec<Vec<Wrapping<u64>>> {
    let w = a[0].len();
    assert!(a.iter().all(|v| v.len() == w));
    (0..w)
        .map(|j| a.iter().map(|v| v[j]).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn hash_rows(a: &[Vec<Wrapping<u64>>], q: usize, base: Wrapping<u64>) -> Vec<Vec<Wrapping<u64>>> {
    let base_pow_q = pow(q, base);
    a.iter()
        .map(|v| {
            let mut hash = hash_seq(&v[..q], base);
            let mut ans = vec![hash];
            for i in 0..v.len() - q {
                hash = hash * base + v[i + q] - v[i] * base_pow_q;
                ans.push(hash);
            }
            ans
        })
        .collect()
}

fn hash_table(table: &[Vec<Wrapping<u64>>], p: usize, q: usize) -> Vec<Vec<Wrapping<u64>>> {
    let table = hash_rows(&table, q, BASE_J);
    let table = transpose(&table);
    let table = hash_rows(&table, p, BASE_I);
    transpose(&table)
}

fn hash_char_ref(c: &char) -> Wrapping<u64> {
    match *c {
        '0' => Wrapping(0),
        '*' => Wrapping(1),
        _ => panic!(),
    }
}

fn main() {
    input!(
        h: usize,
        _w: usize,
        s: [Chars; h],
        p: usize,
        q: usize,
        k: usize,
        patterns: [[Chars; q]; k]
    );

    let mut set: HashSet<Wrapping<u64>> = patterns
        .iter()
        .map(|pattern| {
            let pattern = pattern
                .iter()
                .map(|v| v.iter().map(hash_char_ref).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let table = hash_table(&pattern, p, q);
            assert_eq!(table.len(), 1);
            assert_eq!(table[0].len(), 1);
            table[0][0]
        })
        .collect();

    let s = s
        .iter()
        .map(|v| v.iter().map(hash_char_ref).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let table = hash_table(&s, p, q);
    for x in table.iter().flatten() {
        set.remove(x);
    }

    println!("{}", k - set.len());
}

#[cfg(test)]
mod chap4_7_astrological_sign_tests {
    const BIN: &str = "chap4_7_astrological_sign";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3 3
*00
0**
*00
2 2 2
**
00
*0
**
"#,
            "1\n",
        );
    }
}
