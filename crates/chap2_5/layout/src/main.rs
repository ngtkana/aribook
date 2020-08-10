use proconio::{input, marker::Usize1};

fn main() {
    input!(
        n: usize,
        near_cnt: usize,
        far_cnt: usize,
        near: [(Usize1, Usize1, i32); near_cnt],
        far: [(Usize1, Usize1, i32); far_cnt]
    );
    let mut edges = Vec::new();
    (0..n - 1).for_each(|i| edges.push((i + 1, i, 0)));
    for (mut u, mut v, w) in near {
        if u > v {
            std::mem::swap(&mut u, &mut v);
        }
        edges.push((u, v, w));
    }
    for (mut u, mut v, w) in far {
        if u > v {
            std::mem::swap(&mut u, &mut v);
        }
        edges.push((v, u, -w));
    }

    let mut dist = vec![std::i32::MAX; n];
    dist[0] = 0;
    fn relax(edges: &[(usize, usize, i32)], dist: &mut [i32]) -> bool {
        let mut update = false;
        for &(u, v, w) in edges {
            let ndv = dist[u].saturating_add(w);
            if ndv < dist[v] {
                update = true;
                dist[v] = ndv;
            }
        }
        update
    }

    let negative_cycle = (0..n).map(|_| relax(&edges, &mut dist)).last().unwrap();
    println!(
        "{}",
        if negative_cycle {
            -2
        } else if dist[n - 1] == std::i32::MAX {
            -1
        } else {
            dist[n - 1]
        }
    );
}

#[cfg(test)]
mod chap2_5_layout_tests {
    const BIN: &str = "chap2_5_layout";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"4 2 1
1 3 10
2 4 20
2 3 3
"#,
            "27\n",
        );
    }
}
