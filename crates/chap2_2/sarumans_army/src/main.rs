use proconio::input;

fn main() {
    input!(n: usize, d: u32, a: [u32; n]);
    let mut ans = 0;
    let mut i = 0;
    while i < n {
        let l = a[i];
        while i + 1 < n && a[i + 1] <= l + d {
            i += 1;
        }
        let c = a[i];
        while i + 1 < n && a[i + 1] <= c + d {
            i += 1;
        }
        i += 1;
        ans += 1;
    }
    println!("{}", ans);
}

#[cfg(test)]
mod samples {
    const BIN: &str = "chap2_2_sarumans_army";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"6 10
1 7 15 20 30 50
"#,
            "3\n",
        );
    }
}
