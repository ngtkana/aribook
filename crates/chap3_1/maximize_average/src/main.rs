use proconio::input;

#[allow(clippy::many_single_char_names)]
fn main() {
    input!(n: usize, k: usize, wv: [(f64, f64); n]);
    let mut l = 1e-6f64;
    let mut r = 1e6f64;
    for _ in 0..54 {
        let c = (l * r).sqrt();
        let mut b = wv.iter().map(|&(w, v)| v - c * w).collect::<Vec<_>>();
        b.sort_by_key(|&x| std::cmp::Reverse(ordered_float::NotNan::new(x).unwrap()));
        if 0.0 <= b[0..k].iter().sum::<f64>() {
            l = c;
        } else {
            r = c;
        }
    }
    println!("{:.2}", l);
}

#[cfg(test)]
mod chap3_1_maximize_average_tests {
    const BIN: &str = "chap3_1_maximize_average";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3 2
2 2
5 3
2 1
"#,
            "0.75\n",
        );
    }
}
