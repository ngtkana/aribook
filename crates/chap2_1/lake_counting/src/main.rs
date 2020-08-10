use proconio::input;
// dbg {{{
#[allow(dead_code)]
mod dbg {
    use std::fmt::{Debug, Formatter};

    #[derive(Clone)]
    pub struct Tabular<'a, T: Debug>(pub &'a [Vec<T>]);
    impl<'a, T: Debug> Debug for Tabular<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            for i in 0..self.0.len() {
                writeln!(f, "{:2} | {:?}", i, &self.0[i])?;
            }
            Ok(())
        }
    }

    #[derive(Clone)]
    pub struct BooleanTable<'a>(pub &'a [Vec<bool>]);
    impl<'a> Debug for BooleanTable<'a> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            for i in 0..self.0.len() {
                writeln!(f, "{:2} | {:?}", i, BooleanSlice(&self.0[i]))?;
            }
            Ok(())
        }
    }

    #[derive(Clone)]
    pub struct BooleanSlice<'a>(pub &'a [bool]);
    impl<'a> Debug for BooleanSlice<'a> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(
                f,
                "{}",
                self.0
                    .iter()
                    .map(|&b| if b { '#' } else { '.' })
                    .collect::<String>()
            )?;
            Ok(())
        }
    }
}
// }}}

fn main() {
    input!(h: i32, w:i32, mut field: [proconio::marker::Chars; h as usize]);

    fn dfs(x: i32, y: i32, h: i32, w: i32, field: &mut [Vec<char>]) {
        field[x as usize][y as usize] = '.';
        for dx in -1..=1 {
            for dy in -1..=1 {
                let nx = x + dx;
                let ny = y + dy;
                if 0 <= nx && nx < h && 0 <= ny && ny < w && field[nx as usize][ny as usize] == 'W'
                {
                    dfs(nx, ny, h, w, field);
                }
            }
        }
    }

    let mut res = 0;
    for x in 0..h {
        for y in 0..w {
            if field[x as usize][y as usize] == 'W' {
                dfs(x, y, h, w, &mut field);
                res += 1;
            }
        }
    }
    println!("{}", res);
}

#[cfg(test)]
mod samples {
    const BIN: &str = "chap2_1_lake_counting";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"10 12
W........WW.
.WWW.....WWW
....WW...WW.
.........WW.
.........WW.
.........W..
.........W..
..W......W..
.W.W.....WW.
W.W.W.....W.
.W.W......W.
..W.......W.
"#,
            "3\n",
        );
    }
}
