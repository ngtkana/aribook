#![allow(clippy::many_single_char_names)]
use proconio::input;

fn dfs(a: &mut [u32]) -> u64 {
    let n = a.len();
    if n <= 1 {
        0
    } else {
        let (left, right) = a.split_at_mut(n / 2);
        let mut ans = dfs(left) + dfs(right);
        let mut swp = Vec::new();
        let mut i = 0;
        let mut j = 0;
        while i < left.len() || j < right.len() {
            if j == right.len() || i < left.len() && left[i] <= right[j] {
                swp.push(left[i]);
                i += 1;
            } else {
                ans += left.len() as u64 - i as u64;
                swp.push(right[j]);
                j += 1;
            }
        }
        a.copy_from_slice(&swp);
        ans
    }
}

fn main() {
    input!(n: usize, mut a: [u32; n]);
    println!("{}", dfs(&mut a));
}

#[cfg(test)]
mod chap4_6_number_of_swaps_of_bubble_sort_tests {
    const BIN: &str = "chap4_6_number_of_swaps_of_bubble_sort";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"4
3 1 4 2
"#,
            "3\n",
        );
    }
}
