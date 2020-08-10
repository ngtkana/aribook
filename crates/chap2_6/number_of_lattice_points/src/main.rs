use proconio::input;

fn gcd(mut x: u32, mut y: u32) -> u32 {
    if x < y {
        std::mem::swap(&mut x, &mut y);
    }
    while y != 0 {
        let q = x / y;
        let r = x - q * y;
        x = y;
        y = r;
    }
    x
}

fn main() {
    input!(x0: i32, y0: i32, x1: i32, y1: i32);
    println!(
        "{}",
        gcd((x1 - x0).abs() as u32, (y1 - y0).abs() as u32) + 1
    );
}

#[cfg(test)]
mod chap2_6_number_of_lattice_points_tests {
    const BIN: &str = "chap2_6_number_of_lattice_points";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"1 11
5 13
"#,
            "3\n",
        );
    }
}
