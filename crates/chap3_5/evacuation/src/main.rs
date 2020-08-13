#![allow(clippy::many_single_char_names)]
use proconio::input;
const GRID_FOUR_DIRS_I32: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn main() {
    input!(h: usize, w: usize, grid: [proconio::marker::Chars; h]);
    let mut doors = Vec::new();
    for door_i in 0..h {
        for door_j in (0..w).filter(|&door_j| grid[door_i][door_j] == 'D') {
            let mut dist = vec![vec![std::u32::MAX; w]; h];
            dist[door_i][door_j] = 0;
            let mut queue = std::collections::VecDeque::from(vec![(door_i, door_j)]);
            while let Some((i, j)) = queue.pop_front() {
                for (di, dj) in GRID_FOUR_DIRS_I32.iter() {
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if (0..h as i32).contains(&ni) && (0..w as i32).contains(&nj) {
                        let ni = ni as usize;
                        let nj = nj as usize;
                        if dist[ni][nj] == std::u32::MAX && grid[ni][nj] == '.' {
                            dist[ni][nj] = dist[i][j] + 1;
                            queue.push_back((ni, nj));
                        }
                    }
                }
            }
            doors.push(
                dist.iter()
                    .flatten()
                    .zip(grid.iter().flatten())
                    .filter_map(|(&dist, &c)| if c == '.' { Some(dist) } else { None })
                    .collect::<Vec<_>>(),
            );
        }
    }

    let a = doors.len();
    let b = doors[0].len();
    let s = a + b;
    let t = a + b + 1;

    let orig_r = h as u32 * w as u32 + 1;
    let mut l = 0;
    let mut r = orig_r;
    while 1 < r - l {
        let c = l + (r - l) / 2;
        let mut dinic = chap3_5_dinic::Dinic::with_len(a + b + 2);
        (0..a).for_each(|i| dinic.add_edge(s, i, c as u32));
        (0..b).for_each(|i| dinic.add_edge(a + i, t, 1));
        for (i, door) in doors.iter().enumerate() {
            for (j, &d) in door.iter().enumerate() {
                if d <= c {
                    dinic.add_edge(i, a + j, 1);
                }
            }
        }
        if b as u32 == dinic.run(s, t) {
            r = c;
        } else {
            l = c;
        }
    }
    let ans = r;
    if ans == orig_r {
        println!("impossible");
    } else {
        println!("{}", ans);
    }
}

#[cfg(test)]
mod chap3_5_evacuation_tests {
    const BIN: &str = "chap3_5_evacuation";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"5 5
XXDXX
X...X
D...X
X...D
XXXXX
"#,
            "3\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"5 12
XXXXXXXXXXXX
X..........D
X.XXXXXXXXXX
X..........X
XXXXXXXXXXXX
"#,
            "21\n",
        );
    }

    #[test]
    fn sample3() {
        test_sample(
            r#"5 5
XDXXX
X.X.D
XX.XX
D.X.X
XXXDX
"#,
            "impossible\n",
        );
    }
}
