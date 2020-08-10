use proconio::input;

#[allow(clippy::many_single_char_names)]
fn main() {
    input!(h: usize, w: usize, g: [proconio::marker::Chars; h]);
    let find_char = |c: char| -> (usize, usize) {
        for (i, v) in g.iter().enumerate() {
            for (j, &x) in v.iter().enumerate() {
                if x == c {
                    return (i, j);
                }
            }
        }
        unreachable!()
    };
    let (sx, sy) = find_char('S');
    let (tx, ty) = find_char('G');
    let mut queue = std::collections::VecDeque::from(vec![(sx, sy)]);
    let mut dist = vec![vec![std::u32::MAX; w]; h];
    dist[sx][sy] = 0;
    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == (tx, ty) {
            break;
        }
        let x = x as i32;
        let y = y as i32;
        for (i, j) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
            let nx = x + i;
            let ny = y + j;
            if 0 <= nx
                && nx < h as i32
                && 0 <= ny
                && ny < w as i32
                && g[nx as usize][ny as usize] != '#'
                && dist[nx as usize][ny as usize] == std::u32::MAX
            {
                let x = x as usize;
                let y = y as usize;
                let nx = nx as usize;
                let ny = ny as usize;
                queue.push_back((nx, ny));
                dist[nx][ny] = dist[x][y] + 1;
            }
        }
    }
    println!("{}", dist[tx][ty]);
}

#[cfg(test)]
mod samples {
    const BIN: &str = "chap2_1_shortest_path_in_maze";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"10 10
#S######.#
......#..#
.#.##.##.#
.#........
##.##.####
....#....#
.#######.#
....#.....
.####.###.
....#...G#
"#,
            "22\n",
        );
    }
}
