#![allow(clippy::many_single_char_names)]
use proconio::input;

fn main() {
    input!(n: usize, a: [u32; n]);
    let xor = a
        .rchunks(2)
        .map(|v| match *v {
            [x, y] => y - x - 1,
            [x] => x - 1,
            _ => unreachable!(),
        })
        .fold(0, std::ops::BitXor::bitxor);
    println!(
        "{}",
        if xor == 0 {
            "Bob will win"
        } else {
            "Georgia will win"
        }
    );
}

#[cfg(test)]
mod chap4_2_georgia_and_bob_tests {
    const BIN: &str = "chap4_2_georgia_and_bob";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3
1 2 3
"#,
            "Bob will win\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"8
1 5 6 7 9 12 14 17
"#,
            "Georgia will win\n",
        );
    }
}
