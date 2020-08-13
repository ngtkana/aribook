#![allow(clippy::many_single_char_names)]
use proconio::input;

fn main() {
    input!(mut n: u64);
    let mut a = [[3, 1], [5, 3]];
    let mut ans = [[1, 0], [0, 1]];
    fn mul(a: &[[u32; 2]; 2], b: &[[u32; 2]; 2]) -> [[u32; 2]; 2] {
        let mut c = [[0; 2]; 2];
        for (ai, ci) in a.iter().zip(c.iter_mut()) {
            for (x, bj) in ai.iter().zip(b.iter()) {
                for (y, z) in bj.iter().zip(ci.iter_mut()) {
                    *z += x * y;
                }
            }
        }
        c.iter_mut()
            .map(|v| v.iter_mut())
            .flatten()
            .for_each(|x| *x %= 1000);
        c
    }
    while 0 != n {
        if n % 2 == 1 {
            ans = mul(&ans, &a);
        }
        a = mul(&a, &a);
        n /= 2;
    }
    println!("{:03}", (ans[0][0] * 2 - 1) % 1000);
}

#[cfg(test)]
mod chap3_7_numbers_tests {
    const BIN: &str = "chap3_7_numbers";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample("2\n", "027\n");
    }

    #[test]
    fn sample2() {
        test_sample("5\n", "935\n");
    }
}
