#![allow(clippy::many_single_char_names)]
use proconio::input;

fn main() {
    input!(n: usize, mut xy: [(i32, i32); n]);
    xy.sort();
    let mut adj = vec![vec![false; n]; n];
    for (i, &(_, yi)) in xy.iter().enumerate() {
        for j in [yi - 1, yi, yi + 1].iter().filter_map(|&yi_pm_1| {
            xy[i + 1..]
                .iter()
                .position(|&(_, yj)| yi_pm_1 == yj)
                .map(|pos| i + 1 + pos)
        }) {
            adj[i][j] = true;
            adj[j][i] = true;
        }
    }

    let ans = || -> u32 {
        // 辺がなければ 1 です。
        if adj.iter().flatten().all(|x| !x) {
            return 1;
        }
        // 三角形がなければ 2 です。
        let triangle_exists = (0..n).any(|i| {
            (0..n)
                .filter(|&j| adj[i][j])
                .any(|j| (0..n).any(|k| adj[i][k] && adj[k][j]))
        });
        if !triangle_exists {
            return 2;
        }

        // 3 彩色に挑戦です。
        let mut color = vec![None; n];
        let mut ckd = vec![vec![false; n]; n];
        for (i, j) in (0..n)
            .map(|i| {
                (0..n)
                    .filter(|&j| adj[i][j])
                    .map(|j| (i, j))
                    .collect::<Vec<_>>()
            })
            .flatten()
        {
            assert_eq!(color[i].is_some(), color[j].is_some());
            if color[i].is_some() {
                continue;
            }
            color[i] = Some(1);
            color[j] = Some(2);
            let mut queue = std::collections::VecDeque::from(vec![(i, j)]);
            while let Some((i, j)) = queue.pop_front() {
                let (i, j) = (i.min(j), i.max(j));
                if std::mem::replace(&mut ckd[i][j], true) {
                    continue;
                }
                if let Some(k) = (0..n).find(|&k| adj[i][k] && adj[k][j]) {
                    let want = color[i].unwrap() ^ color[j].unwrap();
                    if let Some(color_k) = color[k] {
                        if color_k != want {
                            return 4;
                        }
                    }
                    color[k] = Some(want);
                    queue.push_back((i, k));
                    queue.push_back((j, k));
                }
            }
        }
        3
    }();
    println!("{}", ans);
}

#[cfg(test)]
mod chap4_8_football_team_tests {
    const BIN: &str = "chap4_8_football_team";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"3
10 10
8 15
12 7
"#,
            "1\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"5
1 1
2 1
3 1
4 1
5 1
"#,
            "2\n",
        );
    }

    #[test]
    fn sample3() {
        test_sample(
            r#"3
1 1
2 2
3 1
"#,
            "3\n",
        );
    }
}
