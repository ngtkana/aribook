use proconio::input;

fn main() {
    input!(n: usize, s: [u32; n], t: [u32; n]);
    let mut tasks = s.iter().copied().zip(t.iter().copied()).collect::<Vec<_>>();
    tasks.sort_by_key(|&(_, y)| y);
    let mut ans = 0;
    let mut reserved = 0;
    for (x, y) in tasks {
        if x <= reserved {
            continue;
        }
        ans += 1;
        reserved = y;
    }
    println!("{}", ans);
}

#[cfg(test)]
mod samples {
    const BIN: &str = "chap2_2_interval_scheduling_problem";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"5
1 2 4 6 8
3 5 7 9 10
"#,
            "3\n",
        );
    }

    #[test]
    fn hack_for_2() {
        test_sample(
            r#"4
1 2 3 5
8 4 6 7
"#,
            "2\n",
        );
    }

    #[test]
    fn hack_for_3() {
        test_sample(
            r#"12
1  2 3 3 3 5 7  9  11 11 11 13
16 4 6 6 6 8 10 12 14 14 14 15
"#,
            "4\n",
        );
    }
}
