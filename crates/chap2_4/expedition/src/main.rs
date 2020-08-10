use proconio::input;

fn main() {
    input!(n: usize, total: u64, mut rest: u64, a: [u64; n], b: [u64; n]);
    let mut heap = std::collections::BinaryHeap::new();
    let mut pos = 0;
    let mut ans = 0;
    for (x, y) in a
        .iter()
        .copied()
        .zip(b.iter().copied())
        .chain(std::iter::once((total, 0)))
    {
        while pos + rest < x {
            if let Some(z) = heap.pop() {
                ans += 1;
                rest += z;
            } else {
                println!("-1");
                std::process::exit(0);
            }
        }
        heap.push(y);
        rest -= x - pos;
        pos = x;
    }
    println!("{}", ans);
}

#[cfg(test)]
mod chap2_4_expedition_tests {
    const BIN: &str = "chap2_4_expedition";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"4 25 10
10 14 20 21
10 5 2 4
"#,
            "2\n",
        );
    }

    #[test]
    fn handmade_impossible() {
        test_sample(
            r#"3 25 10
10 20 23
10 2 4
"#,
            "-1\n",
        );
    }
}
