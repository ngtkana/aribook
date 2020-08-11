#![allow(clippy::many_single_char_names)]
use proconio::input;

fn update(mut i: usize, x: u32, seg: &mut [u32]) {
    let n = (seg.len() + 1) / 2;
    i += n - 1;
    seg[i] = x;
    while 0 != i {
        i = (i - 1) / 2;
        seg[i] = seg[2 * i + 1].min(seg[2 * i + 2]);
    }
}

fn fold(a: usize, b: usize, k: usize, l: usize, r: usize, seg: &[u32]) -> u32 {
    if r <= a || b <= l {
        std::u32::MAX
    } else if a <= l && r <= b {
        seg[k]
    } else {
        let c = (l + r) / 2;
        fold(a, b, 2 * k + 1, l, c, seg).min(fold(a, b, 2 * k + 2, c, r, seg))
    }
}

fn main() {
    input!(n: usize, q: usize, a: [u32; n]);
    let n = n.next_power_of_two();
    let mut seg = vec![std::u32::MAX; 2 * n - 1];
    seg[n - 1..n - 1 + a.len()].copy_from_slice(a.as_slice());
    for i in (0..n - 1).rev() {
        seg[i] = seg[2 * i + 1].min(seg[2 * i + 2]);
    }
    for _ in 0..q {
        input!(command: String);
        match command.as_ref() {
            "update" => {
                input!(i: usize, x: u32);
                update(i, x, &mut seg);
            }
            "fold" => {
                input!(l: usize, r: usize);
                println!("{}", fold(l, r, 0, 0, n, &seg));
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod chap3_3_segtree_tests {
    const BIN: &str = "chap3_3_segtree";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"5 5
1 3 1 3 1
fold 3 4
fold 3 5
update 2 5
fold 2 3
fold 0 3
fold 2 3
"#,
            r#"3
1
5
1
"#,
        );
    }
}
