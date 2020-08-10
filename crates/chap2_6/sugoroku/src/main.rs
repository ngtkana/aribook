use proconio::input;

fn solve_impl(x: u32, y: u32) -> Option<(i32, i32)> {
    if y == 0 {
        if x == 1 {
            Some((1, 0))
        } else {
            None
        }
    } else {
        solve_impl(y, x % y).map(|(u, v)| (v, u - (x / y) as i32 * v))
    }
}

fn solve(mut x: u32, mut y: u32) -> Option<(i32, i32)> {
    let reverse = x < y;
    if reverse {
        std::mem::swap(&mut x, &mut y);
    }
    solve_impl(x, y).map(|(mut u, mut v)| {
        if reverse {
            std::mem::swap(&mut u, &mut v);
        }
        (u, v)
    })
}

fn main() {
    input!(a: u32, b: u32);
    if let Some((x, y)) = solve(a, b) {
        let px = (x.abs() + x) / 2;
        let mx = (x.abs() - x) / 2;
        let py = (y.abs() + y) / 2;
        let my = (y.abs() - y) / 2;
        println!("{} {} {} {}", px, py, mx, my);
    } else {
        println!("-1");
    }
}

#[cfg(test)]
mod chap2_6_sugoroku_tests {
    const BIN: &str = "chap2_6_sugoroku";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"4 11
"#,
            "3 0 0 1\n",
        );
    }
}
