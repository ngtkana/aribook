#![allow(clippy::many_single_char_names)]
pub fn lcp(a: &[u8], sa: &[usize]) -> Vec<usize> {
    let mut rank = vec![0; a.len() + 1];
    for (i, &x) in sa.iter().enumerate() {
        rank[x] = i;
    }

    let mut lcp = vec![0; a.len()];
    let mut h = 0usize;
    for (i, &rank_i) in rank[..a.len()].iter().enumerate() {
        let j = sa[rank_i - 1];
        h = (h.saturating_sub(1)..)
            .find(|&h| {
                a.get(i + h)
                    .and_then(|x| a.get(j + h).map(|y| x != y))
                    .unwrap_or(true)
            })
            .unwrap();
        lcp[rank_i - 1] = h;
    }

    lcp
}

#[cfg(test)]
mod chap4_7_lcp_tests {
    use super::*;
    use chap4_7_suffix_array::suffix_array;

    #[test]
    fn test_abracadabra() {
        let input = "abracadabra";
        let sa = suffix_array(input.as_bytes());
        let result = lcp(input.as_bytes(), &sa);
        let expedted = [0, 1, 4, 1, 1, 0, 3, 0, 0, 0, 2];
        assert_eq!(result.as_slice(), &expedted);
    }

    #[test]
    fn test_dndnd_n_d_n_() {
        let input = "dndnd_n_d_n_";
        let sa = suffix_array(input.as_bytes());
        let result = lcp(input.as_bytes(), &sa);
        let expedted = [0, 3, 1, 4, 0, 2, 1, 2, 0, 1, 1, 3];
        assert_eq!(result.as_slice(), &expedted);
    }
}
