#![allow(clippy::many_single_char_names)]
use proconio::input;

#[derive(Debug, Clone)]
struct Anime {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, Copy)]
enum Event {
    Start(usize),
    End(usize),
}

fn main() {
    input!(n: usize, week: usize, mut st: [(usize, usize); n]);

    for (s, t) in st.iter_mut() {
        if s > t {
            *t += week
        }
    }
    let anime = st
        .iter()
        .map(|&(start, end)| Anime { start, end })
        .chain(
            st.iter()
                .map(|&(s, t)| (s + week, t + week))
                .map(|(start, end)| Anime { start, end }),
        )
        .collect::<Vec<_>>();

    let mut events = (0..2 * n)
        .map(|i| vec![Event::Start(i), Event::End(i)])
        .flatten()
        .collect::<Vec<_>>();
    events.sort_by_key(|e| match *e {
        Event::Start(i) => anime[i].start * 2 + 1,
        Event::End(i) => anime[i].end * 2,
    });

    let mut next = vec![None; 2 * n];
    let mut last = None;
    for e in events.iter().rev() {
        match *e {
            Event::Start(i) => {
                last = Some(i);
            }
            Event::End(i) => {
                next[i] = last;
            }
        }
    }

    let ans = (0..n)
        .map(|i| {
            std::iter::successors(Some(i), |&j| next[j])
                .take_while(|&j| anime[j].end <= anime[i].start + week)
                .count()
        })
        .max()
        .unwrap();

    println!("{}", ans);
}

#[cfg(test)]
mod chap4_4_cyclic_scheduling_next_tests {
    const BIN: &str = "chap4_4_cyclic_scheduling_next";

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
