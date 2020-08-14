#![allow(clippy::many_single_char_names)]
use proconio::input;

fn main() {
    input!(n: u32);
    println!("{}", if n <= 2 { "Alice" } else { "Bob" });
}

#[cfg(test)]
mod chap4_2_a_funny_game_tests {
    const BIN: &str = "chap4_2_a_funny_game";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample("1", "Alice\n");
    }

    #[test]
    fn sample2() {
        test_sample("3", "Bob\n");
    }
}
