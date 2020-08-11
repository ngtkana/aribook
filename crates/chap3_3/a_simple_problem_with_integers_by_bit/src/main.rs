use chap3_3_binary_indexed_tree::BinaryIndexedTree;
use proconio::input;

#[allow(clippy::many_single_char_names)]
fn main() {
    input!(n: usize, q: usize);
    let mut zeroth = BinaryIndexedTree::with_len(n);
    let mut first = BinaryIndexedTree::with_len(n);
    for _ in 0..q {
        input!(s: String);
        match s.as_ref() {
            "add" => {
                input!(l: usize, r: usize, x: i32);
                zeroth.add(l, l as i32 * -x);
                zeroth.add(r, r as i32 * x);
                first.add(l, x);
                first.add(r, -x);
            }
            "sum" => {
                input!(l: usize, r: usize);
                println!(
                    "{}",
                    zeroth.sum(l..r) - l as i32 * first.sum(..l) + r as i32 * first.sum(..r)
                );
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod chap3_3_a_simple_problem_with_integers_by_bit_tests {
    const BIN: &str = "chap3_3_a_simple_problem_with_integers_by_bit";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"5 6
add 1 4 10
sum 3 5
add 0 2 20
sum 0 4
sum 0 5
sum 4 5
"#,
            r#"10
70
70
0
"#,
        );
    }
}
