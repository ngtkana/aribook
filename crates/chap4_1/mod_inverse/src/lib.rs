#![allow(clippy::many_single_char_names)]

pub fn extgcd(a: u64, b: u64, x: &mut i64, y: &mut i64) -> u64 {
    if b != 0 {
        let d = extgcd(b, a % b, y, x);
        *y -= (a / b) as i64 * *x;
        d
    } else {
        *x = 1;
        *y = 0;
        a
    }
}

pub fn mod_inverse(a: u64, m: u64) -> u64 {
    let mut x = 0;
    let mut y = 0;
    extgcd(a, m, &mut x, &mut y);
    x %= m as i64;
    if 0 <= x {
        x as u64
    } else {
        (x + m as i64) as u64
    }
}

#[cfg(test)]
mod chap4_1_mod_inverse_tests {
    use super::*;

    #[test]
    fn test_hand() {
        assert_eq!(mod_inverse(0, 2), 0);
        assert_eq!(mod_inverse(1, 2), 1);

        assert_eq!(mod_inverse(0, 3), 0);
        assert_eq!(mod_inverse(1, 3), 1);
        assert_eq!(mod_inverse(2, 3), 2);

        assert_eq!(mod_inverse(0, 4), 0);
        assert_eq!(mod_inverse(1, 4), 1);
        assert_eq!(mod_inverse(2, 4), 1);
        assert_eq!(mod_inverse(3, 4), 3);

        assert_eq!(mod_inverse(0, 5), 0);
        assert_eq!(mod_inverse(1, 5), 1);
        assert_eq!(mod_inverse(2, 5), 3);
        assert_eq!(mod_inverse(3, 5), 2);
        assert_eq!(mod_inverse(4, 5), 4);

        assert_eq!(mod_inverse(0, 6), 0);
        assert_eq!(mod_inverse(1, 6), 1);
        assert_eq!(mod_inverse(2, 6), 1);
        assert_eq!(mod_inverse(3, 6), 1);
        assert_eq!(mod_inverse(4, 6), 5);
        assert_eq!(mod_inverse(5, 6), 5);

        assert_eq!(mod_inverse(1, 7), 1);
        assert_eq!(mod_inverse(2, 7), 4);
        assert_eq!(mod_inverse(3, 7), 5);
        assert_eq!(mod_inverse(4, 7), 2);
        assert_eq!(mod_inverse(5, 7), 3);
        assert_eq!(mod_inverse(6, 7), 6);

        assert_eq!(mod_inverse(1, 8), 1);
        assert_eq!(mod_inverse(2, 8), 1);
        assert_eq!(mod_inverse(3, 8), 3);
        assert_eq!(mod_inverse(4, 8), 1);
        assert_eq!(mod_inverse(5, 8), 5);
        assert_eq!(mod_inverse(6, 8), 7);
        assert_eq!(mod_inverse(7, 8), 7);
    }
}
