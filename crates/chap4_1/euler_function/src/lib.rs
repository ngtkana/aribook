#![allow(clippy::many_single_char_names)]
pub fn euler_phi(mut n: u32) -> u32 {
    let mut res = n;
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            res -= res / p;
            while n % p == 0 {
                n /= p;
            }
        }
        p += 1;
    }
    if n != 1 {
        res -= res / n;
    }
    res
}

#[cfg(test)]
mod chap4_1_eular_function_tests {
    use super::*;

    #[test]
    fn test_hand() {
        // OEIS A000010 https://oeis.org/search?q=euler+phi&sort=&language=&go=Search
        let expected = [
            1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4, 12, 6, 8, 8, 16, 6, 18, 8, 12, 10, 22, 8, 20, 12,
            18, 12, 28, 8, 30, 16, 20, 16, 24, 12, 36, 18, 24, 16, 40, 12, 42, 20, 24, 22, 46, 16,
            42, 20, 32, 24, 52, 18, 40, 24, 36, 28, 58, 16, 60, 30, 36, 32, 48, 20, 66, 32, 44,
        ];
        expected.iter().zip(1..).for_each(|(&ex, i)| {
            println!("Calculating Euler phi of {}... ( should be {} )", i, ex);
            let result = euler_phi(i);
            assert_eq!(ex, result);
        });
    }
}
