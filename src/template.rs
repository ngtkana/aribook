use proconio::input;

fn main() {
    input!(s: String);
    println!("Hello, {}", s);
}

#[cfg(test)]
mod samples {
    const BIN: &str = "@BIN";

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
