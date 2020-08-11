pub fn enumerate_adjacent_free_subsets(n: usize) -> Vec<u32> {
    let mut bs = 0i32;
    let mut ans = Vec::new();
    while bs < 1 << n {
        ans.push(bs as u32);
        let adjacent_zeroes = !(bs | bs >> 1);
        let lowest_adjacent_zero = adjacent_zeroes & -adjacent_zeroes;
        bs = (bs & !(lowest_adjacent_zero - 1)) | lowest_adjacent_zero;
    }
    ans
}

#[cfg(test)]
mod chap3_2_enumerate_adjacent_free_subsets_tests {
    use super::*;

    #[test]
    fn test_enumerate_adjacent_free_subsets() {
        assert_eq!(enumerate_adjacent_free_subsets(0), vec![0]);
        assert_eq!(enumerate_adjacent_free_subsets(1), vec![0, 1]);
        assert_eq!(enumerate_adjacent_free_subsets(2), vec![0, 1, 2]);
        assert_eq!(enumerate_adjacent_free_subsets(3), vec![0, 1, 2, 4, 5]);
        assert_eq!(
            enumerate_adjacent_free_subsets(4),
            vec![0, 1, 2, 4, 5, 8, 9, 10]
        );
        assert_eq!(
            enumerate_adjacent_free_subsets(5),
            vec![0, 1, 2, 4, 5, 8, 9, 10, 16, 17, 18, 20, 21]
        );
        assert_eq!(
            enumerate_adjacent_free_subsets(6),
            vec![0, 1, 2, 4, 5, 8, 9, 10, 16, 17, 18, 20, 21, 32, 33, 34, 36, 37, 40, 41, 42]
        );
    }
}
