use proconio::input;

fn main() {
    input!(n: usize, m: u32, a: [u32; n]);
    for &x in &a {
        for &y in &a {
            for &z in &a {
                for &w in &a {
                    if x + y + z + w == m {
                        println!("Yes");
                        std::process::exit(0);
                    }
                }
            }
        }
    }
    println!("No");
}

#[cfg(test)]
mod samples {
    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, "sandbox_1_1_1_kujibiki");
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3 10
1 3 5
"#,
            "Yes\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"3 9
1 3 5
"#,
            "No\n",
        );
    }
}
