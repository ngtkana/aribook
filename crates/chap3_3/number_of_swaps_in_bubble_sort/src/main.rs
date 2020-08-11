use proconio::input;

fn main() {
    input!(n: usize, a: [proconio::marker::Usize1; n]);
    let mut bit = chap3_3_binary_indexed_tree::BinaryIndexedTree::with_len(n);
    let mut ans = 0;
    for x in a {
        ans += bit.sum(x..);
        bit.add(x, 1);
    }
    println!("{}", ans);
}

#[cfg(test)]
mod chap3_3_number_of_swaps_in_bubble_sort_tests {
    const BIN: &str = "chap3_3_number_of_swaps_in_bubble_sort";

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
