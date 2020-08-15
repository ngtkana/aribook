#![allow(clippy::many_single_char_names)]
use proconio::input;

fn main() {
    input!(n: usize, k: usize, a: [u32; n]);
    let mut dequeue = std::collections::VecDeque::new();
    let mut ans = Vec::new();
    for (i, &x) in a.iter().enumerate() {
        while let Some(j) = dequeue.pop_back() {
            if a[j] < x {
                dequeue.push_back(j);
                break;
            }
        }
        dequeue.push_back(i);
        if k <= i + 1 {
            ans.push(a[dequeue.pop_front().unwrap()]);
        }
    }
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .fold(String::new(), |acc, x| if acc.is_empty() {
                x
            } else {
                acc + " " + x.as_ref()
            })
    );
}

#[cfg(test)]
mod chap4_4_sliding_minimum_tests {
    const BIN: &str = "chap4_4_sliding_minimum";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"5 3
1 3 5 4 2
"#,
            "1 3 2\n",
        );
    }
}
