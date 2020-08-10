use proconio::input;

fn main() {
    input!(n: usize);
    let mut sieve = vec![false; n + 1];
    for i in (2..).take_while(|&i| i * i <= n) {
        if sieve[i] {
            continue;
        }
        for j in (2..).take_while(|&j| i * j <= n) {
            sieve[i * j] = true;
        }
    }
    println!("{}", sieve[2..].iter().filter(|&b| !b).count());
}

#[cfg(test)]
mod chap2_6_sieve_of_eatosthenes_tests {
    const BIN: &str = "chap2_6_sieve_of_eatosthenes";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"11
"#, "5\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"1000000
"#,
            "78498\n",
        );
    }
}
