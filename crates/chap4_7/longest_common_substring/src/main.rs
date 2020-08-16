#![allow(clippy::many_single_char_names)]
use chap4_7_lcp::lcp;
use chap4_7_suffix_array::suffix_array;
use proconio::input;

fn main() {
    input!(s: String, t: String);
    let l = s.as_bytes().len();
    let st = s
        .as_bytes()
        .iter()
        .copied()
        .chain(std::iter::once(0))
        .chain(t.as_bytes().iter().copied())
        .collect::<Vec<_>>();
    let sa = suffix_array(st.as_slice());
    let lcp = lcp(st.as_slice(), &sa);
    let ans = lcp
        .iter()
        .enumerate()
        .filter_map(|(i, x)| {
            if sa[i] != l && sa[i + 1] != l && (sa[i] < l) != (sa[i + 1] < l) {
                Some(x)
            } else {
                None
            }
        })
        .max()
        .unwrap();
    println!("{}", ans);
}

#[cfg(test)]
mod chap4_7_longest_common_substring_tests {
    const BIN: &str = "chap4_7_longest_common_substring";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"ABRACADABRA
ECADADABRBCRDAR
"#,
            "5\n",
        );
    }
}
