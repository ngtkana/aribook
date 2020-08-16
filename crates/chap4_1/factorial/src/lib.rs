#![allow(clippy::many_single_char_names)]
use chap4_1_mod_inverse::mod_inverse;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Factorial {
    table: Vec<u64>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FactorialResult {
    pub frac: u64,
    pub power: u64,
}

impl Factorial {
    pub fn with_modulus(modulus: u64) -> Self {
        let len = modulus as usize;
        let mut table = vec![1; len];
        for i in 1..len {
            table[i] = (table[i - 1] * i as u64) % modulus;
        }
        Self { table }
    }

    fn power_part(&self, mut n: u64) -> u64 {
        let p = self.table.len() as u64;
        let mut ans = 0;
        while n != 0 {
            n /= p;
            ans += n;
        }
        ans
    }

    fn frac_part(&self, n: u64) -> u64 {
        if n == 0 {
            1
        } else {
            let p = self.table.len() as u64;
            let lower_prt = self.frac_part(n / p);
            let residual_part = self.table[(n % p) as usize];
            let fermat_part = match (n / p) % 2 {
                0 => 1,
                1 => p - 1,
                _ => unreachable!(),
            };
            lower_prt * residual_part * fermat_part % p
        }
    }

    pub fn calc(&self, n: u64) -> FactorialResult {
        FactorialResult {
            frac: self.frac_part(n),
            power: self.power_part(n),
        }
    }

    pub fn binom(&self, n: u64, k: u64) -> FactorialResult {
        let p = self.table.len() as u64;
        let FactorialResult {
            frac: f0,
            power: p0,
        } = self.calc(n);
        let FactorialResult {
            frac: f1,
            power: p1,
        } = self.calc(k);
        let FactorialResult {
            frac: f2,
            power: p2,
        } = self.calc(n - k);
        FactorialResult {
            frac: (f0 * mod_inverse(f1, p) * mod_inverse(f2, p)) % p,
            power: p0 - p1 - p2,
        }
    }
}

#[cfg(test)]
mod chap4_1_factorial_tests {
    use super::*;

    #[test]
    fn test_factorial_p_2() {
        let factorial = Factorial::with_modulus(2);
        assert_eq!(factorial.calc(0), FactorialResult { frac: 1, power: 0 });
        assert_eq!(factorial.calc(1), FactorialResult { frac: 1, power: 0 });
        assert_eq!(factorial.calc(2), FactorialResult { frac: 1, power: 1 });
        assert_eq!(factorial.calc(3), FactorialResult { frac: 1, power: 1 });
        assert_eq!(factorial.calc(4), FactorialResult { frac: 1, power: 3 });
        assert_eq!(factorial.calc(5), FactorialResult { frac: 1, power: 3 });
        assert_eq!(factorial.calc(6), FactorialResult { frac: 1, power: 4 });
    }

    #[test]
    fn test_factorial_p_3() {
        let factorial = Factorial::with_modulus(3);
        assert_eq!(factorial.calc(0), FactorialResult { frac: 1, power: 0 });
        assert_eq!(factorial.calc(1), FactorialResult { frac: 1, power: 0 });
        assert_eq!(factorial.calc(2), FactorialResult { frac: 2, power: 0 });
        assert_eq!(factorial.calc(3), FactorialResult { frac: 2, power: 1 });
        assert_eq!(factorial.calc(4), FactorialResult { frac: 2, power: 1 });
        assert_eq!(factorial.calc(5), FactorialResult { frac: 1, power: 1 });
        assert_eq!(factorial.calc(6), FactorialResult { frac: 2, power: 2 });
    }

    #[test]
    fn test_binomial_p_2() {
        let p = 2;
        let frac = Factorial::with_modulus(p);
        let expected = [
            vec![(1, 0)],
            vec![(1, 0), (1, 0)],
            vec![(1, 0), (1, 1), (1, 0)],
            vec![(1, 0), (1, 0), (1, 0), (1, 0)],
            vec![(1, 0), (1, 2), (1, 1), (1, 2), (1, 0)],
        ];
        let n = expected.len();
        for i in 0..n {
            for j in 0..i {
                let (a, e) = expected[i][j];
                println!(
                    "Calculating binom( {i}, {j} ) ... ( should be {a} × {p} ^ {e} )",
                    i = i,
                    j = j,
                    a = a,
                    e = e,
                    p = p
                );
                let result = frac.binom(i as u64, j as u64);
                assert_eq!(FactorialResult { frac: a, power: e }, result);
            }
        }
    }

    #[test]
    fn test_binomial_p_3() {
        let p = 3;
        let frac = Factorial::with_modulus(p);
        let expected = [
            vec![(1, 0)],
            vec![(1, 0), (1, 0)],
            vec![(1, 0), (2, 0), (1, 0)],
            vec![(1, 0), (1, 1), (1, 1), (1, 0)],
            vec![(1, 0), (1, 0), (2, 1), (1, 0), (1, 0)],
        ];
        let n = expected.len();
        for i in 0..n {
            for j in 0..i {
                let (a, e) = expected[i][j];
                println!(
                    "Calculating binom( {i}, {j} ) ... ( should be {a} × {p} ^ {e} )",
                    i = i,
                    j = j,
                    a = a,
                    e = e,
                    p = p
                );
                let result = frac.binom(i as u64, j as u64);
                assert_eq!(FactorialResult { frac: a, power: e }, result);
            }
        }
    }
}
