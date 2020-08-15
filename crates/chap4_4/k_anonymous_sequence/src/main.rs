#![allow(clippy::many_single_char_names)]
use proconio::input;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
struct LinearFunction {
    a: i64,
    b: i64,
}

impl LinearFunction {
    pub fn from_coeffs(a: i64, b: i64) -> Self {
        Self { a, b }
    }
    pub fn eval(&self, x: i64) -> i64 {
        self.a * x + self.b
    }
    pub fn det_of_three_lines(line_0: &Self, line_1: &Self, line_2: &Self) -> i64 {
        let a = line_1.a - line_0.a;
        let b = line_1.b - line_0.b;
        let c = line_2.a - line_0.a;
        let d = line_2.b - line_0.b;
        a * d - b * c
    }
}

impl Debug for LinearFunction {
    fn fmt<'a>(&self, f: &mut Formatter<'a>) -> std::fmt::Result {
        write!(f, "{}x+{}", self.a, self.b)
    }
}

fn main() {
    input!(n: usize, k: usize, a: [i64; n]);
    let mut sum = vec![0; n + 1];
    (0..n).for_each(|i| sum[i + 1] = sum[i] + a[i]);

    let mut deque = std::collections::VecDeque::from(vec![LinearFunction::from_coeffs(-a[0], 0)]);
    let mut queue = std::collections::VecDeque::new();

    let ans = || -> i64 {
        for (i, &x) in a
            .iter()
            .enumerate()
            .skip(k)
            .chain(std::iter::once((n, &std::i64::MAX)))
        {
            while 2 <= deque.len() && deque[0].eval(i as i64) >= deque[1].eval(i as i64) {
                deque.pop_front().unwrap();
            }

            let dp_i = deque.front().unwrap().eval(i as i64) + sum[i];
            if i == n {
                return dp_i;
            }
            queue.push_back(LinearFunction::from_coeffs(
                -x,
                dp_i + i as i64 * x - sum[i],
            ));

            if k <= queue.len() {
                let line_2 = queue.pop_front().unwrap();
                if line_2.a < deque.back().unwrap().a {
                    while 2 <= deque.len()
                        && LinearFunction::det_of_three_lines(
                            &deque[deque.len() - 2],
                            &deque[deque.len() - 1],
                            &line_2,
                        ) < 0
                    {
                        deque.pop_back();
                    }
                    deque.push_back(line_2);
                }
            }
        }
        unreachable!()
    }();
    println!("{}", ans);
}

#[cfg(test)]
mod chap4_4_k_anonymous_sequence_tests {
    const BIN: &str = "chap4_4_k_anonymous_sequence";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"7 3
2 2 3 4 4 5 5
"#,
            "3\n",
        );
    }
}
