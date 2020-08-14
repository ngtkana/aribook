#![allow(clippy::many_single_char_names)]
pub fn gcd(a: u64, b: u64) -> u64 {
    if b != 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

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

// ax + b = 0 mod m
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LinearCongruenceEquation {
    pub a: u64,
    pub b: u64,
    pub m: u64,
}

impl LinearCongruenceEquation {
    pub fn with_coeffs_and_modulus(a: u64, b: u64, m: u64) -> Self {
        LinearCongruenceEquation { a, b, m }
    }
    pub fn solve(&self) -> Option<CongruenceRepresentative> {
        let mut x = 0;
        let mut y = 0;
        let g = extgcd(self.a, self.m, &mut x, &mut y);
        if self.b % g != 0 {
            None
        } else {
            let b = self.b / g;
            let m = self.m / g;
            Some(CongruenceRepresentative {
                value: (m as i64 - x) as u64 * b % m,
                modulus: m,
            })
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CongruenceRepresentative {
    value: u64,
    modulus: u64,
}

pub fn solve_linear_congruence_equations(
    equations: &[LinearCongruenceEquation],
) -> Option<CongruenceRepresentative> {
    let mut now = CongruenceRepresentative {
        value: 0,
        modulus: 1,
    };
    for &LinearCongruenceEquation { a, b, m } in equations {
        let eq = LinearCongruenceEquation {
            a: a * now.modulus,
            b: b + a * now.value,
            m,
        };
        let CongruenceRepresentative {
            value: t,
            modulus: m,
        } = eq.solve()?;
        now.value += t * now.modulus;
        now.modulus *= m;
        now.value %= now.modulus;
    }
    Some(now)
}

#[cfg(test)]
mod chap4_1_modulo_equations_tests {
    use super::*;

    #[test]
    fn test_hand1() {
        let eq0 = LinearCongruenceEquation::with_coeffs_and_modulus(3, 4, 7);
        let expected = CongruenceRepresentative {
            value: 1,
            modulus: 7,
        };

        println!(
            r#"Solving
    {}x + {} = 0 (mod {})
        (should be {} mod {})"#,
            eq0.a, eq0.b, eq0.m, expected.value, expected.modulus
        );

        let result = solve_linear_congruence_equations(&[eq0]).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hand2() {
        let eq0 = LinearCongruenceEquation { a: 2, b: 2, m: 8 };
        let eq1 = LinearCongruenceEquation { a: 3, b: 3, m: 6 };
        let expected = CongruenceRepresentative {
            value: 3,
            modulus: 4,
        };

        println!(
            r#"Solving
    {a0}x + {b0} = 0 (mod {m0})
    {a1}x + {b1} = 0 (mod {m1})
        (should be {expected_value} mod {expected_modulus})"#,
            a0 = eq0.a,
            b0 = eq0.b,
            m0 = eq0.m,
            a1 = eq1.a,
            b1 = eq1.b,
            m1 = eq1.m,
            expected_value = expected.value,
            expected_modulus = expected.modulus
        );
        let result = solve_linear_congruence_equations(&[eq0, eq1]).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hand3() {
        let eq0 = LinearCongruenceEquation { a: 10, b: 1, m: 11 };
        let eq1 = LinearCongruenceEquation { a: 12, b: 1, m: 13 };
        let eq2 = LinearCongruenceEquation { a: 16, b: 1, m: 17 };
        let expected = CongruenceRepresentative {
            value: 1,
            modulus: 2431,
        };

        println!(
            r#"Solving
    {a0}x + {b0} = 0 (mod {m0})
    {a1}x + {b1} = 0 (mod {m1})
    {a2}x + {b2} = 0 (mod {m2})
        (should be {expected_value} mod {expected_modulus})"#,
            a0 = eq0.a,
            b0 = eq0.b,
            m0 = eq0.m,
            a1 = eq1.a,
            b1 = eq1.b,
            m1 = eq1.m,
            a2 = eq2.a,
            b2 = eq2.b,
            m2 = eq2.m,
            expected_value = expected.value,
            expected_modulus = expected.modulus
        );
        let result = solve_linear_congruence_equations(&[eq0, eq1, eq2]).unwrap();
        assert_eq!(result, expected);
    }
}
