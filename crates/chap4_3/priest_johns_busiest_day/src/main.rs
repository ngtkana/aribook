#![allow(clippy::many_single_char_names)]
use chap4_3_scc::Scc;
use proconio::input;

fn decode_time(s: &str) -> u32 {
    let mut s = s.split(':');
    let a = s.next().unwrap().parse::<u32>().unwrap();
    let b = s.next().unwrap().parse::<u32>().unwrap();
    a * 60 + b
}

fn format_time(x: u32) -> String {
    format!("{:02}:{:02}", x / 60, x % 60)
}

fn main() {
    input!(n: usize, std: [(String, String, u32); n]);
    let intervals = std
        .iter()
        .map(|(s, t, d)| {
            let s = decode_time(&s);
            let t = decode_time(&t);
            [(s, s + d), (t - d, t)]
        })
        .collect::<Vec<_>>();

    let mut scc = Scc::with_len(2 * n);
    for (i, i_slice) in intervals.iter().enumerate() {
        for (j, j_slice) in intervals.iter().enumerate().skip(i + 1) {
            for (k, &(i_start, i_end)) in i_slice.iter().enumerate() {
                for (l, &(j_start, j_end)) in j_slice.iter().enumerate() {
                    if i_start.max(j_start) < i_end.min(j_end) {
                        scc.add_edge((1 - k) * n + i, l * n + j);
                        scc.add_edge(k * n + i, (1 - l) * n + j);
                    }
                }
            }
        }
    }

    let cmp = Scc::convert_into_belongingness_table(scc.run());
    if cmp[..n].iter().zip(cmp[n..].iter()).any(|(&x, &y)| x == y) {
        println!("NO");
    } else {
        use std::cmp::Ordering;
        println!("YES");
        for ((&x, &y), &[(a, b), (c, d), ..]) in
            cmp[..n].iter().zip(cmp[n..].iter()).zip(intervals.iter())
        {
            let (start, end) = match x.cmp(&y) {
                Ordering::Greater => (format_time(a), format_time(b)),
                Ordering::Less => (format_time(c), format_time(d)),
                Ordering::Equal => unreachable!(),
            };
            println!("{} {}", start, end);
        }
    }
}

#[cfg(test)]
mod chap4_3_priest_joens_busiest_day_tests {
    const BIN: &str = "chap4_3_priest_johns_busiest_day";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"2
08:00 09:00 30
08:15 09:00 20
"#,
            r#"YES
08:00 08:30
08:40 09:00
"#,
        );
    }
}
