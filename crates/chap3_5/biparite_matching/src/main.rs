#![allow(clippy::many_single_char_names)]
use proconio::input;

fn main() {
    input!(a: usize, b: usize, m: usize, uv: [(usize, usize); m]);
    let mut dinic = chap3_5_dinic::Dinic::with_len(a + b + 2);
    let s = a + b;
    let t = a + b + 1;
    (0..a).for_each(|i| dinic.add_edge(s, i, 1));
    (0..b).for_each(|i| dinic.add_edge(a + i, t, 1));
    uv.iter().for_each(|&(u, v)| dinic.add_edge(u, a + v, 1));
    println!("{}", dinic.run(s, t));
}

#[cfg(test)]
mod chap3_5_biparite_matching_tests {
    const BIN: &str = "chap3_5_biparite_matching";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3 3 5
0 0
0 1
1 0
1 2
2 1
"#,
            "3\n",
        );
    }
}
