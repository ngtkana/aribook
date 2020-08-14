#![allow(clippy::many_single_char_names)]
use proconio::input;

fn main() {
    input!(n: usize, k: usize, a: [usize; k], b: [usize; n]);
    let b_max = *b.iter().max().unwrap();
    let mut grundy = vec![0; b_max + 1];
    for i in 1..=b_max {
        let mut ckd = vec![false; k + 1];
        for &x in a.iter().filter(|&&x| x <= i) {
            let g = grundy[i - x];
            if g < k {
                ckd[g] = true;
            }
        }
        grundy[i] = ckd.iter().enumerate().find(|&(_, &x)| !x).unwrap().0;
    }
    let xor = b
        .iter()
        .map(|&x| grundy[x])
        .fold(0, std::ops::BitXor::bitxor);
    println!("{}", if xor == 0 { "Bob" } else { "Alice" });
}

#[cfg(test)]
mod chap4_2_coin_game_2_tests {
    const BIN: &str = "chap4_2_coin_game_2";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3 3
1 3 4
5 6 7
"#,
            "Alice\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"3 3
1 3 4
5 6 8
"#,
            "Bob\n",
        );
    }
}
