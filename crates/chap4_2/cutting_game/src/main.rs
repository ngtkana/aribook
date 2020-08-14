#![allow(clippy::many_single_char_names)]
use proconio::input;

fn main() {
    input!(h: usize, w: usize);
    let mut grundy = vec![vec![0; w + 1]; h + 1];
    for i in 2..=h {
        for j in 2..=w {
            let len = i / 2 + j / 2;
            let mut ckd = vec![false; len + 1];
            for g in (2..=i / 2)
                .map(|i0| grundy[i0][j] ^ grundy[i - i0][j])
                .chain((2..=j / 2).map(|j0| grundy[i][j0] ^ grundy[i][j - j0]))
            {
                if g < len {
                    ckd[g] = true;
                }
            }
            grundy[i][j] = ckd.iter().enumerate().find(|&(_, x)| !x).unwrap().0;
        }
    }
    println!("{}", if grundy[h][w] == 0 { "LOSE" } else { "WIN" });
}

#[cfg(test)]
mod chap4_2_cutting_game_tests {
    const BIN: &str = "chap4_2_cutting_game";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample("2 2", "LOSE\n");
    }

    #[test]
    fn sample2() {
        test_sample("4 2", "WIN\n");
    }

    #[test]
    fn sample3() {
        test_sample("60 60", "WIN\n");
    }
}
