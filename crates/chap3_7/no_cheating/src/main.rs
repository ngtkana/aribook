#![allow(clippy::many_single_char_names)]
use chap3_5_dinic::Dinic;
use proconio::input;

fn main() {
    input!(h: usize, w: usize, grid: [proconio::marker::Chars; h]);
    let mut coord = [vec![], vec![]];
    for (i, j) in (0..h)
        .map(|i| {
            std::iter::repeat(i).zip(0..w).filter_map(|(i, j)| {
                if grid[i][j] == '.' {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
        .flatten()
    {
        coord[j % 2].push((i, j));
    }
    coord.iter_mut().for_each(|v| v.sort());
    let a = coord[0].len();
    let b = coord[1].len();
    let s = a + b;
    let t = s + 1;
    let mut dinic = Dinic::with_len(t + 1);
    (0..a).for_each(|i| dinic.add_edge(s, i, 1));
    (0..b).for_each(|i| dinic.add_edge(a + i, t, 1));

    for parity in 0..2 {
        let coord_p = &coord[parity];
        let coord_q = &coord[1 - parity];
        for (k, &(i, j)) in coord_p.iter().enumerate() {
            for l in [(-1, -1), (-1, 1), (0, -1), (0, 1)]
                .iter()
                .filter_map(|(di, dj)| {
                    use std::convert::TryInto;
                    let ni = (i as i32 + di).try_into().ok()?;
                    let nj = (j as i32 + dj).try_into().ok()?;
                    coord_q.binary_search(&(ni, nj)).ok()
                })
            {
                match parity {
                    0 => dinic.add_edge(k, a + l, 1),
                    1 => dinic.add_edge(l, a + k, 1),
                    _ => unreachable!(),
                }
            }
        }
    }

    println!("{}", a + b - dinic.run(s, t));
}

#[cfg(test)]
mod chap3_7_no_cheating_tests {
    const BIN: &str = "chap3_7_no_cheating";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"2 3
...
...
"#,
            "4\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"2 3
x.x
xxx
"#,
            "1\n",
        );
    }

    #[test]
    fn sample3() {
        test_sample(
            r#"2 3
x.x
x.x
"#,
            "2\n",
        );
    }
}
