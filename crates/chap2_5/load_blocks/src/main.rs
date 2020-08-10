use proconio::{input, marker::Usize1};
use std::cmp::Reverse;
// dbg {{{
#[allow(dead_code)]
mod dbg {
    use std::fmt::{Debug, Formatter};

    #[derive(Clone)]
    pub struct Tabular<'a, T: Debug>(pub &'a [T]);
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
    input!(n: usize, m: usize, uvw: [(Usize1, Usize1, u32); m]);
    let mut g = vec![vec![]; n];
    for (u, v, w) in uvw {
        g[u].push((v, w));
        g[v].push((u, w));
    }
    let mut dist = vec![(std::u32::MAX, std::u32::MAX); n];
    let mut heap = std::collections::BinaryHeap::from(vec![(Reverse(0), 0)]);
    dist[0] = (0, std::u32::MAX);
    while let Some((Reverse(dx), x)) = heap.pop() {
        if dist[x].1 < dx {
            continue;
        }
        for &(y, w) in g[x].iter() {
            let mut ndy = dx + w;
            if ndy < dist[y].0 {
                heap.push((Reverse(ndy), y));
                std::mem::swap(&mut ndy, &mut dist[y].0);
            }
            if dist[y].0 < ndy && ndy < dist[y].1 {
                heap.push((Reverse(ndy), y));
                dist[y].1 = ndy;
            }
        }
    }
    println!("{}", dist[n - 1].1);
}

#[cfg(test)]
mod chap2_5_load_blocks_tests {
    const BIN: &str = "chap2_5_load_blocks";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"4 4
1 2 100
2 3 250
2 4 200
3 4 100
"#,
            "450\n",
        );
    }
}
