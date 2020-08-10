use proconio::input;

fn main() {
    input!(s: String);
    println!("Hello, {}", s);
}

#[cfg(test)]
mod @CHAP_@PROBLEM_tests {
    const BIN: &str = "@CHAP_@PROBLEM";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"chokudai
"#,
            "Hello, chokudai\n",
        );
    }
}
