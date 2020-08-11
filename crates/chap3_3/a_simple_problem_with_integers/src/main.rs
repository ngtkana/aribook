use proconio::input;

#[allow(clippy::many_single_char_names)]
fn main() {
    input!(n: usize, q: usize);

    // セグメント木
    let mut data = vec![0; 3 * n + 5];
    let mut datb = vec![0; 3 * n + 5];

    // [a, b) に x を加算する
    // kは節点の番号で、区間[l, r)に対応する
    #[allow(clippy::many_single_char_names, clippy::too_many_arguments)]
    fn add(
        a: usize,
        b: usize,
        x: i64,
        k: usize,
        l: usize,
        r: usize,
        data: &mut [i64],
        datb: &mut [i64],
    ) {
        if a <= l && r <= b {
            data[k] += x;
        } else if l < b && a < r {
            datb[k] += (r.min(b) - l.max(a)) as i64 * x;
            let c = (l + r) / 2;
            add(a, b, x, k * 2 + 1, l, c, data, datb);
            add(a, b, x, k * 2 + 2, c, r, data, datb);
        }
    }

    // [a, b) の和を計算する
    // kは節点の番号で、区間[l, r)に対応する
    #[allow(clippy::many_single_char_names)]
    fn sum(a: usize, b: usize, k: usize, l: usize, r: usize, data: &[i64], datb: &[i64]) -> i64 {
        if b <= l || r <= a {
            0
        } else if a <= l && r <= b {
            data[k] * (r - l) as i64 + datb[k]
        } else {
            let c = (l + r) / 2;
            (r.min(b) - l.max(a)) as i64 * data[k]
                + sum(a, b, k * 2 + 1, l, c, data, datb)
                + sum(a, b, k * 2 + 2, c, r, data, datb)
        }
    }

    for _ in 0..q {
        input!(command: String);
        match command.as_ref() {
            "add" => {
                input!(l: usize, r: usize, x: i64);
                add(l, r, x, 0, 0, n, &mut data, &mut datb);
            }
            "sum" => {
                input!(l: usize, r: usize);
                println!("{}", sum(l, r, 0, 0, n, &data, &datb));
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod chap3_3_a_simple_problem_with_integers_tests {
    const BIN: &str = "chap3_3_a_simple_problem_with_integers";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"5 6
add 1 4 10
sum 3 5
add 0 2 20
sum 0 4
sum 0 5
sum 4 5
"#,
            r#"10
70
70
0
"#,
        );
    }
}
