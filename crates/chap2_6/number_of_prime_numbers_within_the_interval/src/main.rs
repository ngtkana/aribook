use proconio::input;

fn main() {
    input!(l: usize, r: usize);
    let root_r = (r as f64).sqrt() as usize;
    let mut small_sieve = vec![false; root_r + 1];
    let mut large_sieve = vec![false; r - l];
    for p in 2..root_r {
        if small_sieve[p] {
            continue;
        }
        for i in (2..).take_while(|&i| p * i <= root_r) {
            small_sieve[p * i] = true;
        }
        for i in ((l + p - 1) / p..).take_while(|&i| p * i <= r) {
            large_sieve[(p * i) - l] = true;
        }
    }
    println!("{}", large_sieve.iter().copied().filter(|&b| !b).count());
}

#[cfg(test)]
mod chap2_6_number_of_prime_numbers_within_the_interval_tests {
    const BIN: &str = "chap2_6_number_of_prime_numbers_within_the_interval";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"22 37
"#,
            "3\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"22801763489 22801787297
"#,
            "1000\n",
        );
    }
}
