use proconio::input;

fn main() {
    input!(n: usize, mut a: [u64; n]);
    let mut ans = 0;
    while 1 < a.len() {
        let n = a.len();
        let mut i = (0..n).min_by_key(|&i| a[i]).unwrap();
        let mut j = (0..n).filter(|&j| j != i).min_by_key(|&j| a[j]).unwrap();
        if i > j {
            std::mem::swap(&mut i, &mut j);
        }
        let z = a[i] + a[j];
        a.remove(j);
        a.remove(i);
        a.push(z);
        ans += z;
    }
    println!("{}", ans);
}

#[cfg(test)]
mod samples {
    const BIN: &str = "chap2_2_fence_repair";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3
8 5 8
"#,
            "34\n",
        );
    }

    #[test]
    fn editorial() {
        test_sample(
            r#"5
3 4 5 1 2
"#,
            "33\n",
        );
    }
}
