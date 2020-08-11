pub fn enumerate_combinations(n: usize, k: usize) -> Vec<u32> {
    if k == 0 {
        vec![0]
    } else {
        let mut comb = (1 << k) - 1i32;
        let mut ans = Vec::new();
        while comb < 1 << n {
            ans.push(comb as u32);
            let lowest_bit = comb & -comb;
            let fixed_bits = comb + lowest_bit;
            comb = (((comb & !fixed_bits) / lowest_bit) >> 1) | fixed_bits;
        }
        ans
    }
}

#[cfg(test)]
mod chap3_2_enumerate_combinations_tests {
    use super::*;

    #[test]
    fn test_enumerate_combinatinos() {
        assert_eq!(enumerate_combinations(0, 0), vec![0]);
        assert_eq!(enumerate_combinations(1, 0), vec![0]);
        assert_eq!(enumerate_combinations(1, 1), vec![1]);
        assert_eq!(enumerate_combinations(2, 1), vec![1, 2]);
        assert_eq!(enumerate_combinations(2, 2), vec![3]);
        assert_eq!(enumerate_combinations(3, 1), vec![1, 2, 4]);
        assert_eq!(enumerate_combinations(3, 2), vec![3, 5, 6]);
        assert_eq!(enumerate_combinations(3, 3), vec![7]);
        assert_eq!(enumerate_combinations(4, 1), vec![1, 2, 4, 8]);
        assert_eq!(enumerate_combinations(4, 2), vec![3, 5, 6, 9, 10, 12]);
        assert_eq!(enumerate_combinations(4, 3), vec![7, 11, 13, 14]);
        assert_eq!(enumerate_combinations(4, 4), vec![15]);
    }
}
