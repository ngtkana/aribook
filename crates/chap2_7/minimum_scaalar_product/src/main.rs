use proconio::input;

fn main() {
    input!(n: usize, mut a: [i64; n], mut b: [i64; n]);
    a.sort();
    b.sort();
    println!(
        "{}",
        a.iter()
            .zip(b.iter().rev())
            .map(|(x, y)| x * y)
            .sum::<i64>()
    );
}

#[cfg(test)]
mod chap2_7_minimum_scaalar_product_tests {
    const BIN: &str = "chap2_7_minimum_scaalar_product";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3
1 3 -5
-2 4 1
"#,
            "-25\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"5
1 2 3 4 5
1 0 1 0 1
"#,
            "6\n",
        );
    }
}
