#![allow(clippy::many_single_char_names)]
use proconio::input;

fn main() {
    input!(n: usize, wvm: [(usize, i32, usize); n], cap: usize);
    let mut dp = vec![0; cap + 1];
    for (w, v, m) in wvm {
        for (i, x) in dp.iter_mut().enumerate() {
            *x -= (i / w) as i32 * v
        }
        let mut swp = vec![0; cap + 1];
        for r in 0..w.min(cap + 1) {
            let mut deque = std::collections::VecDeque::new();
            for i in 0..=(cap - r) / w {
                while let Some(j) = deque.pop_back() {
                    if dp[i * w + r] < dp[j * w + r] {
                        deque.push_back(j);
                        break;
                    }
                }
                deque.push_back(i);
                let prv = *deque.front().unwrap();
                swp[i * w + r] = dp[prv * w + r];
                if prv + m == i {
                    deque.pop_front();
                }
            }
        }
        dp = swp;
        for (i, x) in dp.iter_mut().enumerate() {
            *x += (i / w) as i32 * v;
        }
    }
    println!("{}", dp.last().unwrap());
}

#[cfg(test)]
mod chap4_4_bounded_knapsack_tests {
    const BIN: &str = "chap4_4_bounded_knapsack";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3
3 2 5
2 4 1
4 3 3
12
"#,
            "11\n",
        );
    }
}
