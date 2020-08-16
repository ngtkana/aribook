#![allow(clippy::many_single_char_names)]
use chap4_3_sparse_table_argmin::SparseTableArgmin;
use chap4_7_lcp::lcp;
use chap4_7_suffix_array::suffix_array;
use proconio::input;

fn main() {
    input!(s: String);
    let s = s.as_bytes();
    let n = s.len();
    let mut a = s.to_vec();
    a.push(0);
    a.extend(s.iter().rev());

    let sa = suffix_array(&a);
    let mut rank = vec![0; 2 * n + 2];
    for (i, &x) in sa.iter().enumerate() {
        rank[x] = i;
    }

    let lcp = lcp(&a, &sa);
    let sparse_table = SparseTableArgmin::from_vec(lcp);

    let lcp = |i: usize, j: usize| -> usize {
        assert_ne!(i, j);
        let mut u = rank[i];
        let mut v = rank[j];
        if u > v {
            std::mem::swap(&mut u, &mut v);
        }
        sparse_table.values[sparse_table.argmin(u..v)]
    };

    let ans = (1..n)
        .map(|i| 2 * lcp(i, 2 * n + 1 - i))
        .chain((0..n).map(|i| 2 * lcp(i, 2 * n - i) - 1))
        .max()
        .unwrap();

    println!("{}", ans);
}

#[cfg(test)]
mod chap4_7_longest_pelindrome_tests {
    const BIN: &str = "chap4_7_longest_pelindrome";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample("mississippi\n", "7\n");
    }
}
