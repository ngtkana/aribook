use proconio::input;

#[allow(clippy::many_single_char_names)]
fn main() {
    input!(n: usize, a: [u32; n], k: u32);
    println!(
        "{}",
        if k <= a[0] {
            0
        } else {
            let mut l = 0;
            let mut r = n;
            while 1 < r - l {
                let c = l + (r - l) / 2;
                if k <= a[c] {
                    r = c;
                } else {
                    l = c;
                }
            }
            r
        }
    );
}

#[cfg(test)]
mod chap3_1_lower_bound_tests {
    const BIN: &str = "chap3_1_lower_bound";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"5
2 3 3 5 6
3
"#,
            "1\n",
        );
    }
}
