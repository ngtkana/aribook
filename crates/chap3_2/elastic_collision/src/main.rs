use ordered_float::NotNan;
use proconio::input;

const ACCELERATION: f64 = 10f64;

fn calc_height(initial_height: f64, mut time: f64) -> f64 {
    let period = 2.0 * (2.0 * initial_height / ACCELERATION).sqrt();
    time = (time / period).fract() * period;
    time = time.min(period - time);
    initial_height - 0.5 * ACCELERATION * time * time
}

fn main() {
    input!(n: usize, h: f64, mut r: f64, t: f64);
    r *= 0.01;
    let mut ans = (0..n)
        .map(|i| NotNan::new(calc_height(h, t - i as f64)).unwrap())
        .collect::<Vec<_>>();
    ans.sort();
    println!(
        "{}",
        ans.iter()
            .enumerate()
            .map(|(i, x)| (2 * i) as f64 * r + x.into_inner())
            .map(|x| format!("{:.2}", x))
            .fold(String::new(), |acc, x| if acc.is_empty() {
                x
            } else {
                acc + " " + x.as_ref()
            })
    );
}

#[cfg(test)]
mod chap3_2_elastic_collision_tests {
    const BIN: &str = "chap3_2_elastic_collision";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample("1 10 10 100\n", "4.95\n");
    }

    #[test]
    fn sample2() {
        test_sample("2 10 10 100\n", "4.95 10.20\n");
    }
}
