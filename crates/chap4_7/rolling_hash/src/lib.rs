#![allow(clippy::many_single_char_names)]

use std::num::Wrapping;
const BASE: Wrapping<u64> = Wrapping(1_000_000_007);

fn pow(p: usize) -> Wrapping<u64> {
    match p {
        0 => Wrapping(1),
        1 => BASE,
        p => pow(p / 2) * pow((p + 1) / 2),
    }
}

fn hash_byte(c: u8) -> Wrapping<u64> {
    Wrapping(c as u64)
}

fn hash_seq(a: &[u8]) -> Wrapping<u64> {
    let mut ans = Wrapping(0u64);
    for &c in a {
        ans *= BASE;
        ans += hash_byte(c);
    }
    ans
}

pub fn contained_in_bytes(a: &[u8], b: &[u8]) -> bool {
    if a.len() > b.len() {
        false
    } else {
        let base_pow_len_a = pow(a.len());
        let ah = hash_seq(a);
        let mut bh = hash_seq(&b[..a.len()]);
        if ah == bh {
            return true;
        }
        for i in 0..b.len() - a.len() {
            bh = bh * BASE + hash_byte(b[i + a.len()]) - hash_byte(b[i]) * base_pow_len_a;
            if ah == bh {
                return true;
            }
        }
        false
    }
}
pub fn overlap_bytes(a: &[u8], b: &[u8]) -> usize {
    let mut ah = Wrapping(0);
    let mut bh = Wrapping(0);
    let mut coeff = Wrapping(1);
    let mut ans = 0;
    for i in 0..a.len().min(b.len()) {
        ah += hash_byte(a[a.len() - 1 - i]) * coeff;
        bh = bh * BASE + hash_byte(b[i]);
        if ah == bh {
            ans = i + 1;
        }
        coeff *= BASE;
    }
    ans
}

pub fn overlap(a: &str, b: &str) -> usize {
    overlap_bytes(a.as_bytes(), b.as_bytes())
}
pub fn contained_in(a: &str, b: &str) -> bool {
    contained_in_bytes(a.as_bytes(), b.as_bytes())
}

#[cfg(test)]
mod chap4_7_rolling_hash {
    use super::*;

    #[test]
    fn test_contained_in() {
        // true
        assert!(contained_in(&"", &"a"));
        assert!(contained_in(&"a", &"a"));
        assert!(contained_in(&"a", &"ab"));
        assert!(contained_in(&"a", &"ba"));
        assert!(contained_in(&"aa", &"aab"));
        assert!(contained_in(&"ab", &"aab"));

        // false
        assert!(!contained_in(&"a", &""));
        assert!(!contained_in(&"a", &"b"));
        assert!(!contained_in(&"aa", &"a"));
        assert!(!contained_in(&"ab", &"ba"));
        assert!(!contained_in(&"aba", &"ba"));
    }

    #[test]
    fn test_overlap_str() {
        assert_eq!(overlap(&"aaaabb", &"aabbccc"), 4);
    }
}
