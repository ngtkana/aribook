use proconio::input;

fn pow(mut a: u64, mut b: u64, m: u64) -> u64 {
    let mut ans = 1;
    while 0 != b {
        if b % 2 == 1 {
            ans = (ans * a) % m;
        }
        a = (a * a) % m;
        b /= 2;
    }
    ans
}

fn main() {
    input!(n: u64);
    println!(
        "{}",
        if (2..n).any(|x| n % x == 0) && (2..n).all(|x| x == pow(x, n, n)) {
            "Yes"
        } else {
            "No"
        }
    );
}

#[cfg(test)]
mod chap2_6_carmichael_numbers_tests {
    const BIN: &str = "chap2_6_carmichael_numbers";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"17
"#, "No\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"561
"#, "Yes\n",
        );
    }

    #[test]
    fn sample3() {
        test_sample(
            r#"4
"#, "No\n",
        );
    }
}
