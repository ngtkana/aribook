use proconio::input;
use std::cmp::Reverse;

fn main() {
    input!(n: usize, a: [u64; n]);
    let mut heap = a
        .iter()
        .map(|&x| Reverse(x))
        .collect::<std::collections::BinaryHeap<_>>();
    let mut ans = 0;
    while 1 < heap.len() {
        let Reverse(x) = heap.pop().unwrap();
        let Reverse(y) = heap.pop().unwrap();
        ans += x + y;
        heap.push(Reverse(x + y));
    }
    println!("{}", ans);
}

#[cfg(test)]
mod chap2_4_fence_repair_fast_tests {
    const BIN: &str = "chap2_4_fence_repair_fast";

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
