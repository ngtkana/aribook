#![allow(clippy::many_single_char_names)]
use chap4_7_manacher::manacher;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!(s: String);
    let manacher = manacher(
        std::iter::repeat(0)
            .interleave_shortest(s.as_bytes().iter().copied())
            .collect::<Vec<_>>()
            .as_slice(),
    );
    println!("{}", manacher.iter().max().unwrap() - 1);
}

#[cfg(test)]
mod chap4_7_longest_pelindrome_manacher_tests {
    const BIN: &str = "chap4_7_longest_pelindrome_manacher";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample("mississippi\n", "7\n");
    }
}
