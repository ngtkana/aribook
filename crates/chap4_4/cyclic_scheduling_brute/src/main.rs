#![allow(clippy::many_single_char_names)]
use proconio::input;

#[derive(Debug, Clone)]
struct Anime {
    start: usize,
    end: usize,
}

fn main() {
    input!(n: usize, week: usize, mut st: [(usize, usize); n]);
    for (s, t) in st.iter_mut() {
        if s > t {
            *t += week
        }
    }

    let mut anime = st
        .iter()
        .map(|&(start, end)| Anime { start, end })
        .chain(
            st.iter()
                .map(|&(s, t)| (s + week, t + week))
                .map(|(start, end)| Anime { start, end }),
        )
        .collect::<Vec<_>>();
    anime.sort_by_key(|&Anime { end, .. }| end);

    let ans = (0..n)
        .map(|i| {
            let mut count = 0;
            let mut j = i;
            let mut last = 0;
            while anime[j].end <= anime[i].start + week {
                if last <= anime[j].start {
                    count += 1;
                    last = anime[j].end;
                }
                j += 1;
            }
            count
        })
        .max()
        .unwrap();

    println!("{}", ans);
}

#[cfg(test)]
mod chap4_4_cyclic_scheduling_brute_tests {
    const BIN: &str = "chap4_4_cyclic_scheduling_brute";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3 10
0 3
3 7
7 0
"#,
            "3\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"3 10
0 5
3 7
6 9
"#,
            "2\n",
        );
    }
}
