#![allow(clippy::many_single_char_names)]
use proconio::input;

fn main() {
    input!(n: usize, a: [u32; n]);
    println!(
        "{}",
        if a.iter().fold(0, std::ops::BitXor::bitxor) == 0 {
            "Bob"
        } else {
            "Alice"
        }
    );
}

#[cfg(test)]
mod chap4_2_nim_tests {
    const BIN: &str = "chap4_2_nim";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3
1 2 4
"#,
            "Alice\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"3
1 2 3
"#,
            "Bob\n",
        );
    }
}
