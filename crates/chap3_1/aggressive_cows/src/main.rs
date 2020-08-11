use proconio::input;

#[allow(clippy::many_single_char_names)]
fn main() {
    input!(n: usize, k: usize, mut a: [usize; n]);
    a.sort();
    let mut l = 0;
    let mut r = 1 + *a.iter().max().unwrap();
    while 1 < r - l {
        let c = l + (r - l) / 2;
        let mut count = 1;
        let mut i = 0;
        for j in 1..n {
            if i + c <= j {
                i = j;
                count += 1;
            }
        }
        if k <= count {
            l = c;
        } else {
            r = c;
        }
    }
    println!("{}", r);
}

#[cfg(test)]
mod chap3_1_aggressive_cows_tests {
    const BIN: &str = "chap3_1_aggressive_cows";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"5 3
1 2 8 4 9
"#,
            "3\n",
        );
    }
}
