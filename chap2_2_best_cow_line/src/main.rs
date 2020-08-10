use proconio::input;

fn main() {
    input!(n: usize, mut s: proconio::marker::Chars);
    let mut a = s.iter().collect::<std::collections::VecDeque<_>>();
    let mut b = s.iter().rev().collect::<std::collections::VecDeque<_>>();
    let mut ans = Vec::new();
    for _ in 0..n {
        ans.push((if a < b { a.pop_front() } else { b.pop_front() }).unwrap());
    }
    println!("{}", ans.iter().copied().collect::<String>());
}

#[cfg(test)]
mod samples {
    const BIN: &str = "chap2_2_best_cow_line";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"6
ACDBCB
"#,
            "ABCBCD\n",
        );
    }
}
