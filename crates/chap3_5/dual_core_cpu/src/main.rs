use proconio::{input, marker::Usize1};

fn main() {
    input!(n: usize);
    input!(m: usize);
    input!(ab: [(u64, u64); n]);
    input!(abw: [(Usize1, Usize1, u64); m]);
    let s = n;
    let t = s + 1;
    let mut dinic = chap3_5_dinic::Dinic::with_len(t + 1);
    for (i, &(x, y)) in ab.iter().enumerate() {
        dinic.add_edge(s, i, y);
        dinic.add_edge(i, t, x);
    }
    for (a, b, w) in abw {
        dinic.add_edge(a, b, w);
        dinic.add_edge(b, a, w);
    }
    println!("{}", dinic.run(s, t));
}

#[cfg(test)]
mod chap3_5_dual_core_cpu_tests {
    const BIN: &str = "chap3_5_dual_core_cpu";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3 1
1 10
2 10
10 3
2 3 1000
"#,
            "13\n",
        );
    }
}
