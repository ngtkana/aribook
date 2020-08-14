#![allow(clippy::many_single_char_names)]
use proconio::input;

fn wins(mut a: u32, mut b: u32) -> bool {
    if a == 0 || b == 0 {
        false
    } else {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        match b / a {
            0 => unreachable!(),
            1 => !wins(b % a, a),
            _ => true,
        }
    }
}

fn main() {
    input!(a: u32, b: u32);
    println!(
        "{}",
        if wins(a, b) {
            "Stan wins"
        } else {
            "Ollie wins"
        }
    );
}

#[cfg(test)]
mod chap4_2_euclids_game_tests {
    const BIN: &str = "chap4_2_euclids_game";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample("34 12", "Stan wins\n");
    }

    #[test]
    fn sample2() {
        test_sample("15 24", "Ollie wins\n");
    }
}
