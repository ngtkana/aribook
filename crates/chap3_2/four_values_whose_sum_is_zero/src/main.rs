use proconio::input;

fn main() {
    input!(n: usize, a: [[i32; n]; 4]);
    fn enumerate(a: &[i32], b: &[i32]) -> Vec<i32> {
        a.iter()
            .map(|x| b.iter().map(move |y| x + y))
            .flatten()
            .collect()
    }
    let mut hash_map = std::collections::HashMap::new();
    for x in enumerate(&a[2], &a[3]) {
        *hash_map.entry(x).or_insert(0) += 1;
    }
    println!(
        "{}",
        enumerate(&a[0], &a[1])
            .into_iter()
            .map(|x| hash_map.get(&-x).unwrap_or(&0))
            .sum::<u64>()
    );
}

#[cfg(test)]
mod chap3_2_four_values_whose_sum_is_zero_tests {
    const BIN: &str = "chap3_2_four_values_whose_sum_is_zero";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"6
-45 -41 -36 -36 26 -32
22 -27 53 30 -38 -54
42 56 -37 -75 -10 -6
-16 30 77 -46 62 45
"#,
            "5\n",
        );
    }
}
