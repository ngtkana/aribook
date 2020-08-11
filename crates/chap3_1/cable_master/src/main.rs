use proconio::input;

#[allow(clippy::many_single_char_names)]
fn main() {
    input!(n: usize, k: u32, a: [f64; n]);
    let mut l = 1e-6;
    let mut r = a.iter().sum::<f64>();
    for _ in 0..75 {
        let c = (l * r).sqrt();
        if k as f64 <= a.iter().map(|x| (x / c).floor()).sum::<f64>() {
            l = c;
        } else {
            r = c;
        }
    }
    println!("{:.2}", l);
}

#[cfg(test)]
mod chap3_1_cable_master_tests {
    const BIN: &str = "chap3_1_cable_master";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"4 11
8.02 7.43 4.57 5.39
"#,
            "2.00\n",
        );
    }
}
