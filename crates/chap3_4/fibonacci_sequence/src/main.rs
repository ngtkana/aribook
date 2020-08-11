use proconio::input;

const MODULUS: u32 = 10_000;

fn main() {
    input!(n: u64);
    fn pow(mut a: [[u32; 2]; 2], mut b: u64) -> [[u32; 2]; 2] {
        let mut ans = [[1, 0], [0, 1]];
        while 0 != b {
            if b % 2 == 1 {
                ans = mul(&ans, &a);
            }
            a = mul(&a, &a);
            b /= 2;
        }
        ans
    }
    fn mul(a: &[[u32; 2]; 2], b: &[[u32; 2]; 2]) -> [[u32; 2]; 2] {
        let mut c = [[0; 2]; 2];
        for i in 0..2 {
            for j in 0..2 {
                for (k, bk) in b.iter().enumerate() {
                    c[i][j] += a[i][k] * bk[j];
                }
            }
        }
        c.iter_mut()
            .map(|v| v.iter_mut())
            .flatten()
            .for_each(|x| *x %= MODULUS);
        c
    }
    let a = pow([[1, 1], [1, 0]], n);
    println!("{}", a[0][1]);
}

#[cfg(test)]
mod chap3_4_fibonacci_sequence_tests {
    const BIN: &str = "chap3_4_fibonacci_sequence";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"10
"#, "55\n",
        );
    }
}
