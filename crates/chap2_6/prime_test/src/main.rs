use proconio::input;

fn main() {
    input!(n: u32);
    for i in (2..).take_while(|&i| i * i <= n) {
        if n % i == 0 {
            println!("No");
            std::process::exit(0);
        }
    }
    println!("Yes");
}

#[cfg(test)]
mod chap2_6_prime_test_tests {
    const BIN: &str = "chap2_6_prime_test";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"53
"#, "Yes\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"295927
"#,
            "No\n",
        );
    }
}
