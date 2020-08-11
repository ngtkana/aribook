use proconio::input;

const MODULUS: u32 = 10_007;

fn main() {
    input!(n: u64);
    fn pow(mut a: [[u32; 3]; 3], mut b: u64) -> [[u32; 3]; 3] {
        let mut ans = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
        while 0 != b {
            if b % 2 == 1 {
                ans = mul(&ans, &a);
            }
            a = mul(&a, &a);
            b /= 2;
        }
        ans
    }
    fn mul(a: &[[u32; 3]; 3], b: &[[u32; 3]; 3]) -> [[u32; 3]; 3] {
        let mut c = [[0; 3]; 3];
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
    let a = pow([[2, 1, 0], [2, 2, 2], [0, 1, 2]], n);
    println!("{}", a[0][0] + a[2][0]);
}

#[cfg(test)]
mod chap3_4_blocks_tests {
    const BIN: &str = "chap3_4_blocks";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample("1\n", "2\n");
    }

    #[test]
    fn sample2() {
        test_sample("2\n", "6\n");
    }
}
