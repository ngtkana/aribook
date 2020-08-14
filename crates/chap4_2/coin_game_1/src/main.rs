#![allow(clippy::many_single_char_names)]
use proconio::input;

fn main() {
    input!(n: usize, q: usize, a: [usize; q]);
    let mut win = vec![false; n + 1];
    for i in 0..=n {
        if win[i] {
            continue;
        }
        for &x in a.iter().filter(|&&x| i + x <= n) {
            win[i + x] = true;
        }
    }
    println!("{}", if win[n] { "Alice" } else { "Bob" });
}

#[cfg(test)]
mod chap4_2_coin_game_1_tests {
    const BIN: &str = "chap4_2_coin_game_1";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"9 2
1 4
"#,
            "Alice\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"10 2
1 4
"#,
            "Bob\n",
        );
    }
}
