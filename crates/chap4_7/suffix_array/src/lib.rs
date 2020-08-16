#![allow(clippy::many_single_char_names)]
use std::iter::{once, successors};

pub fn suffix_array(a: &[u8]) -> Vec<usize> {
    let mut rank = a
        .iter()
        .map(|&x| x as usize + 1)
        .chain(once(0))
        .collect::<Vec<_>>();
    let mut sa = (0..=a.len()).collect::<Vec<_>>();

    for k in successors(Some(1), |x| Some(x * 2)).take_while(|&x| x <= a.len()) {
        let key = |i: usize| (rank[i], rank.get(i + k));
        sa.sort_by_key(|&i| key(i));
        let swp = (0..a.len())
            .map(|i| key(sa[i]) < key(sa[i + 1]))
            .collect::<Vec<_>>();
        for (i, &b) in swp.iter().enumerate() {
            rank[sa[i + 1]] = if b { rank[sa[i]] + 1 } else { rank[sa[i]] };
        }
    }
    sa
}

#[cfg(test)]
mod chap4_7_suffix_array_tests {
    use super::*;

    #[test]
    fn test_hand_1() {
        let input = "abracadabra";
        let result = suffix_array(input.as_bytes());
        let expected = [11, 10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2];
        assert_eq!(result.as_slice(), &expected);
    }

    // NOTE: よすぽジャッジとあり本で若干定義が違います。
    // よすぽでは、もとの文字列の長さです。
    // あり本では、もとの文字列長さ + 1 です。
    #[test]
    fn test_yosupo_sample_1() {
        let input = "abcbcba";
        let result = suffix_array(input.as_bytes());
        let expected = [7, 6, 0, 5, 3, 1, 4, 2];
        assert_eq!(result.as_slice(), &expected);
    }

    #[test]
    fn test_yosupo_sample_2() {
        let input = "mississippi";
        let result = suffix_array(input.as_bytes());
        let expected = [11, 10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2];
        assert_eq!(result.as_slice(), &expected);
    }

    #[test]
    fn test_yosupo_sample_3() {
        let input = "ababacaca";
        let result = suffix_array(input.as_bytes());
        let expected = [9, 8, 0, 2, 6, 4, 1, 3, 7, 5];
        assert_eq!(result.as_slice(), &expected);
    }

    #[test]
    fn test_yosupo_sample_4() {
        let input = "aaaaa";
        let result = suffix_array(input.as_bytes());
        let expected = [5, 4, 3, 2, 1, 0];
        assert_eq!(result.as_slice(), &expected);
    }
}
