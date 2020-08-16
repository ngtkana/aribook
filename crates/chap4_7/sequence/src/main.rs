#![allow(clippy::many_single_char_names)]
use chap4_7_suffix_array::suffix_array;
use proconio::input;

fn main() {
    input!(n: usize, mut a: [u8; n]);

    // ひとつめの切れ目を探します。
    let mut a_reverse = a.clone();
    a_reverse.reverse();
    let sa = suffix_array(&a_reverse);
    let split_0 = sa
        .iter()
        .copied()
        .map(|x| a.len() - x)
        .find(|&x| 0 != x && x + 2 <= a.len())
        .unwrap();

    // ふたつめの切れ目を探します。
    let mut b = a[split_0..].to_vec();
    b.extend(&a[split_0..]);
    b.reverse();
    let sa = suffix_array(&b);
    let split_1 = sa
        .iter()
        .copied()
        .filter(|&x| x < b.len() / 2)
        .map(|x| a.len() - x)
        .find(|&x| split_0 < x && x < a.len())
        .unwrap();

    // さて作りましょう。
    a[..split_0].reverse();
    a[split_0..split_1].reverse();
    a[split_1..].reverse();
    println!(
        "{}",
        a.iter()
            .map(|x| x.to_string())
            .fold(String::new(), |acc, x| if acc.is_empty() {
                x
            } else {
                acc + " " + x.as_ref()
            })
    );
}

#[cfg(test)]
mod chap4_7_sequence_tests {
    const BIN: &str = "chap4_7_sequence";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"5
10 1 2 3 4
"#,
            "1 10 2 4 3\n",
        );
    }
}
