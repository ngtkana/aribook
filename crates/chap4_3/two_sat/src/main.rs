#![allow(clippy::many_single_char_names)]
use chap4_3_scc::Scc;
use proconio::input;

fn main() {
    input!(n: usize, m: usize, abcd: [(usize, usize, usize, usize); m]);
    let mut scc = Scc::with_len(2 * n);
    for (a, b, c, d) in abcd {
        scc.add_edge((1 - a) * n + b, c * n + d);
        scc.add_edge(a * n + b, (1 - c) * n + d);
    }
    let cmp = Scc::convert_into_belongingness_table(scc.run());
    if cmp[..n].iter().zip(cmp[n..].iter()).any(|(&x, &y)| x == y) {
        println!("NO");
    } else {
        println!("YES");
        for (&x, &y) in cmp[..n].iter().zip(cmp[n..].iter()) {
            println!("{}", if x < y { "true" } else { "false" });
        }
    }
}

#[cfg(test)]
mod chap4_3_two_sat_tests {
    const BIN: &str = "chap4_3_two_sat";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3 3
0 0 1 1
0 1 0 2
1 2 1 0
"#,
            r#"YES
true
true
false
"#,
        );
    }
}
