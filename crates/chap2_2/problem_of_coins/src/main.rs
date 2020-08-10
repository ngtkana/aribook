use proconio::input;

fn main() {
    input!(a: [u32; 6], mut k: u32);
    let b = [1, 5, 10, 50, 100, 500];
    let mut ans = 0;
    for (x, y) in a.iter().copied().zip(b.iter().copied()).rev() {
        let q = (k / y).min(x);
        ans += q;
        k -= y * q;
    }
    println!("{}", ans);
}

#[cfg(test)]
mod samples {
    const BIN: &str = "chap2_2_problem_of_coins";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3 2 1 3 0 2
620
"#,
            "6\n",
        );
    }
}
