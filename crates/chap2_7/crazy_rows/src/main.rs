use proconio::input;

fn main() {
    input!(n: usize, g: [proconio::marker::Chars; n]);
    let mut a = g
        .iter()
        .map(|s| (0..n).rfind(|&j| s[j] == '1').map(|j| j + 1).unwrap_or(0))
        .collect::<Vec<_>>();
    let mut ans = 0;
    for i in 0..n {
        let p = (i..n).find(|&p| a[p] <= i + 1).unwrap();
        ans += p - i;
        a[i..=p].rotate_right(1);
    }
    println!("{}", ans);
}

#[cfg(test)]
mod chap2_7_crazy_rows_tests {
    const BIN: &str = "chap2_7_crazy_rows";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"2
10
11
"#,
            "0\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"3
001
100
010
"#,
            "2\n",
        );
    }

    #[test]
    fn sample3() {
        test_sample(
            r#"4
1110
1100
1100
1000
"#,
            "4\n",
        );
    }
}
