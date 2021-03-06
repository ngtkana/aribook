#![allow(clippy::many_single_char_names)]
// modint {{{
#[allow(dead_code)]
mod modint {
    use std::{
        cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
        fmt::{Debug, Display},
        iter::{Product, Sum},
        mem::swap,
        ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    };

    pub type ModValue = i64;

    #[derive(Debug, Clone, Copy)]
    struct Rational {
        num: ModValue,
        den: ModValue,
    }

    #[allow(clippy::many_single_char_names)]
    fn red(r: i64, p: i64) -> (i64, i64, i64) {
        if r.abs() <= 10000 {
            return (r, 1, 0);
        }
        let mut nxt_r = p % r;
        let mut q = p / r;
        if 2 * nxt_r >= r {
            nxt_r -= r;
            q += 1;
        }
        if 2 * nxt_r <= -r {
            nxt_r += r;
            q -= 1;
        }
        let (x, z, y) = red(nxt_r, r);
        (x, y - q * z, z)
    }

    #[derive(Clone, Copy)]
    pub struct Mint<Mod: ModTrait>(ModValue, std::marker::PhantomData<Mod>);

    impl<Mod: ModTrait> Mint<Mod> {
        fn from_value_unchecked(value: ModValue) -> Self {
            Self(value, std::marker::PhantomData)
        }
        fn normalize(value: ModValue) -> ModValue {
            let value = value % Mod::modulus();
            if 0 <= value {
                value
            } else {
                value + Mod::modulus()
            }
        }
        fn guess(&self) -> Rational {
            let (mut num, mut den, _) = red(self.0, Mod::modulus());
            if den < 0 {
                num = -num;
                den = -den;
            }
            Rational { num, den }
        }
        pub fn from_i64(value: ModValue) -> Self {
            Self::from_value_unchecked(Self::normalize(value))
        }
        pub fn from_frac(num: ModValue, den: ModValue) -> Self {
            Self::from_i64(num) / Self::from_i64(den)
        }
        pub fn zero() -> Self {
            Self::from_value_unchecked(0)
        }
        pub fn one() -> Self {
            Self::from_value_unchecked(1)
        }
        #[allow(clippy::many_single_char_names)]
        pub fn inv(self) -> Self {
            assert_ne!(
                self,
                Self::zero(),
                "attempted to take the inverse of zero mint"
            );
            let mut x = self.0;
            let mut y = Mod::modulus();
            let mut u = 1;
            let mut v = 0;
            while x != 0 {
                let q = y / x;
                y -= q * x;
                v -= q * u;
                swap(&mut x, &mut y);
                swap(&mut u, &mut v);
            }
            assert!(x == 0 && y == 1 && u.abs() == Mod::modulus() && v.abs() < Mod::modulus());
            Self::from_value_unchecked(if v < 0 { v + Mod::modulus() } else { v })
        }
        pub fn pow(mut self, mut p: u64) -> Self {
            let mut ans = Self::one();
            while 0 != p {
                if p % 2 == 1 {
                    ans *= self;
                }
                self *= self;
                p /= 2;
            }
            ans
        }
        pub fn from_pow(a: ModValue, p: u64) -> Self {
            Self::from_i64(a).pow(p)
        }
    }

    impl<Mod: ModTrait> Debug for Mint<Mod> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            let Rational { num, den } = self.guess();
            f.debug_tuple("Mint")
                .field(&if den == 1 {
                    num.to_string()
                } else {
                    format!("{}/{}", num, den)
                })
                .finish()
        }
    }

    impl<Mod: ModTrait> Display for Mint<Mod> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(f, "{}", self.0)
        }
    }

    macro_rules! forward_ref_binop {
        ($(impl $imp:ident, $method:ident)*) => {
            $(
                impl<'a, Mod: ModTrait> $imp<Mint<Mod>> for &'a Mint<Mod> {
                    type Output = Mint<Mod>;

                    #[inline]
                    fn $method(self, other: Mint<Mod>) -> Self::Output {
                        $imp::$method(*self, other)
                    }
                }

                impl<'a, Mod: ModTrait> $imp<&'a Mint<Mod>> for Mint<Mod> {
                    type Output = Mint<Mod>;

                    #[inline]
                    fn $method(self, other: &Mint<Mod>) -> Self::Output {
                        $imp::$method(self, *other)
                    }
                }

                impl<'a, Mod: ModTrait> $imp<&'a Mint<Mod>> for &'a Mint<Mod> {
                    type Output = Mint<Mod>;

                    #[inline]
                    fn $method(self, other: &Mint<Mod>) -> Self::Output {
                        $imp::$method(*self, *other)
                    }
                }
            )*
        };
    }

    macro_rules! forward_ref_op_assign {
        ($(impl $imp:ident, $method:ident)*) => {
            $(
                impl<'a, Mod: ModTrait> $imp<&Mint<Mod>> for Mint<Mod> {
                    #[inline]
                    fn $method(&mut self, other: &Mint<Mod>) {
                        $imp::$method(self, *other);
                    }
                }
            )*
        }
    }

    #[allow(clippy::suspicious_arithmetic_impl)]
    impl<Mod: ModTrait> Add for Mint<Mod> {
        type Output = Self;

        #[inline]
        fn add(self, rhs: Self) -> Self {
            let value = self.0 + rhs.0;
            Self::from_value_unchecked(if value < Mod::modulus() {
                value
            } else {
                value - Mod::modulus()
            })
        }
    }

    #[allow(clippy::suspicious_arithmetic_impl)]
    impl<Mod: ModTrait> Sub for Mint<Mod> {
        type Output = Self;

        #[inline]
        fn sub(self, rhs: Self) -> Self {
            let value = self.0 - rhs.0;
            Self::from_value_unchecked(if 0 <= value {
                value
            } else {
                value + Mod::modulus()
            })
        }
    }

    #[allow(clippy::suspicious_arithmetic_impl)]
    impl<Mod: ModTrait> Mul for Mint<Mod> {
        type Output = Self;

        #[inline]
        fn mul(self, rhs: Self) -> Self {
            Self::from_value_unchecked(self.0 * rhs.0 % Mod::modulus())
        }
    }

    #[allow(clippy::suspicious_arithmetic_impl)]
    impl<Mod: ModTrait> Div for Mint<Mod> {
        type Output = Self;

        #[inline]
        fn div(self, rhs: Self) -> Self {
            self * rhs.inv()
        }
    }

    impl<Mod: ModTrait> Neg for Mint<Mod> {
        type Output = Self;

        #[inline]
        fn neg(self) -> Self {
            if self.0 == 0 {
                Self::zero()
            } else {
                Self::from_value_unchecked(Mod::modulus() - self.0)
            }
        }
    }

    impl<Mod: ModTrait> Neg for &Mint<Mod> {
        type Output = Mint<Mod>;

        #[inline]
        fn neg(self) -> Self::Output {
            (*self).neg()
        }
    }

    macro_rules! forward_assign_biop {
        ($(impl $trait: ident, $fn_assign: ident, $fn: ident)*) => {
            $(
                impl<Mod: ModTrait> $trait for Mint<Mod> {
                    #[inline]
                    fn $fn_assign(&mut self, rhs: Self) {
                        *self = self.$fn(rhs);
                    }
                }
            )*
        };
    }

    forward_assign_biop! {
        impl AddAssign, add_assign, add
        impl SubAssign, sub_assign, sub
        impl MulAssign, mul_assign, mul
        impl DivAssign, div_assign, div
    }

    forward_ref_binop! {
        impl Add, add
        impl Sub, sub
        impl Mul, mul
        impl Div, div
    }

    forward_ref_op_assign! {
        impl AddAssign, add_assign
        impl SubAssign, sub_assign
        impl MulAssign, mul_assign
        impl DivAssign, div_assign
    }

    impl<Mod: ModTrait> PartialEq for Mint<Mod> {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    impl<Mod: ModTrait> Eq for Mint<Mod> {}
    impl<Mod: ModTrait> Ord for Mint<Mod> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.0.cmp(&other.0)
        }
    }
    impl<Mod: ModTrait> PartialOrd for Mint<Mod> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<Mod: ModTrait> From<ModValue> for Mint<Mod> {
        fn from(value: ModValue) -> Self {
            Self::from_i64(value)
        }
    }

    impl<Mod: ModTrait> From<Mint<Mod>> for ModValue {
        fn from(mint: Mint<Mod>) -> Self {
            mint.0
        }
    }

    impl<Mod: ModTrait> Sum for Mint<Mod> {
        fn sum<I>(iter: I) -> Self
        where
            I: Iterator<Item = Self>,
        {
            iter.fold(Self::zero(), Add::add)
        }
    }
    impl<'a, Mod: 'a + ModTrait> Sum<&'a Self> for Mint<Mod> {
        fn sum<I>(iter: I) -> Self
        where
            I: Iterator<Item = &'a Self>,
        {
            iter.fold(Self::zero(), Add::add)
        }
    }
    impl<Mod: ModTrait> Product for Mint<Mod> {
        fn product<I>(iter: I) -> Self
        where
            I: Iterator<Item = Self>,
        {
            iter.fold(Self::one(), Mul::mul)
        }
    }
    impl<'a, Mod: 'a + ModTrait> Product<&'a Self> for Mint<Mod> {
        fn product<I>(iter: I) -> Self
        where
            I: Iterator<Item = &'a Self>,
        {
            iter.fold(Self::one(), Mul::mul)
        }
    }

    #[macro_export]
    macro_rules! mint {
        ($value: expr) => {
            Mint::from_i64($value)
        };
    }

    #[macro_export]
    macro_rules! from_frac {
        ($num: expr, $den: expr) => {
            Mint::from_frac($num, $den)
        };
    }

    #[macro_export]
    macro_rules! from_pow {
        ($a: expr, $b: expr) => {
            Mint::from_pow($a, $b)
        };
    }

    pub trait ModTrait: Clone + Copy + Debug {
        fn modulus() -> ModValue;
    }

    #[derive(Debug, Clone)]
    pub struct Factorial<Mod: ModTrait> {
        normal: Vec<Mint<Mod>>,
        inverse: Vec<Mint<Mod>>,
    }

    impl<Mod: ModTrait> Factorial<Mod> {
        pub fn is_empty(&self) -> bool {
            self.normal.is_empty()
        }
        pub fn len(&self) -> usize {
            self.normal.len()
        }
        pub fn with_len(len: usize) -> Self {
            if len == 0 {
                Self {
                    normal: Vec::new(),
                    inverse: Vec::new(),
                }
            } else {
                let mut normal = vec![Mint::one(); len];
                for i in 1..len {
                    normal[i] = normal[i - 1] * Mint::from_i64(i as i64);
                }
                let mut inverse = vec![normal.last().unwrap().inv(); len];
                for i in (1..len).rev() {
                    inverse[i - 1] = inverse[i] * Mint::from_i64(i as i64);
                }
                Self { normal, inverse }
            }
        }
        pub fn inv(&self, i: usize) -> Mint<Mod> {
            self.inverse[i]
        }
        pub fn falling(&self, n: usize, p: usize) -> Mint<Mod> {
            if p == 0 {
                Mint::one()
            } else if n < p {
                Mint::zero()
            } else {
                self[n] * self.inv(n - p)
            }
        }
        pub fn binom(&self, n: usize, k: usize) -> Mint<Mod> {
            if k == 0 {
                Mint::one()
            } else if n < k {
                Mint::zero()
            } else {
                self[n] * self.inv(n - k) * self.inv(k)
            }
        }
    }

    impl<Mod: ModTrait, I: std::slice::SliceIndex<[Mint<Mod>]>> std::ops::Index<I> for Factorial<Mod> {
        type Output = I::Output;

        #[inline]
        fn index(&self, index: I) -> &Self::Output {
            &self.normal[index]
        }
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Mod100000007 {}
    #[derive(Clone, Copy, Debug)]
    pub struct Mod998244353 {}
    impl ModTrait for Mod100000007 {
        fn modulus() -> ModValue {
            1_000_000_007
        }
    }
    impl ModTrait for Mod998244353 {
        fn modulus() -> ModValue {
            998_244_353
        }
    }
    pub type Mint100000007 = Mint<Mod100000007>;

    pub type Mint998244353 = Mint<Mod998244353>;
}
// }}}
// dbg {{{
#[allow(dead_code)]
mod dbg {
    #[macro_export]
    macro_rules! lg {
        () => {
            $crate::eprintln!("[{}:{}]", $crate::file!(), $crate::line!());
        };
        ($val:expr) => {
            match $val {
                tmp => {
                    eprintln!("[{}:{}] {} = {:?}",
                        file!(), line!(), stringify!($val), &tmp);
                    tmp
                }
            }
        };
        ($val:expr,) => { lg!($val) };
        ($($val:expr),+ $(,)?) => {
            ($(lg!($val)),+,)
        };
    }

    #[macro_export]
    macro_rules! tabular {
        ($val:expr) => {
            eprintln!(
                "[{}:{}] {}:\n{:?}",
                file!(),
                line!(),
                stringify!($val),
                crate::dbg::Tabular($val)
            );
        };
    }

    use std::fmt::{Debug, Formatter};

    #[derive(Clone)]
    pub struct Tabular<'a, T: Debug>(pub &'a [T]);
    impl<'a, T: Debug> Debug for Tabular<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            for i in 0..self.0.len() {
                writeln!(f, "{:2} | {:?}", i, &self.0[i])?;
            }
            Ok(())
        }
    }

    #[derive(Clone)]
    pub struct BooleanTable<'a>(pub &'a [Vec<bool>]);
    impl<'a> Debug for BooleanTable<'a> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            for i in 0..self.0.len() {
                writeln!(f, "{:2} | {:?}", i, BooleanSlice(&self.0[i]))?;
            }
            Ok(())
        }
    }

    #[derive(Clone)]
    pub struct BooleanSlice<'a>(pub &'a [bool]);
    impl<'a> Debug for BooleanSlice<'a> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(
                f,
                "{}",
                self.0
                    .iter()
                    .map(|&b| if b { "1 " } else { "0 " })
                    .collect::<String>()
            )?;
            Ok(())
        }
    }
}
// }}}
type Mint = modint::Mint100000007;
use proconio::input;

fn primes(mut n: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            primes.push(p);
            while n % p == 0 {
                n /= p;
            }
        }
        p += 1;
    }
    if n != 1 {
        primes.push(n);
    }
    primes
}

fn divisors(n: u64) -> Vec<u64> {
    let mut divisors = Vec::new();
    let mut d = 1;
    while d * d <= n {
        if n % d == 0 {
            divisors.push(d);
            if d * d != n {
                divisors.push(n / d);
            }
        }
        d += 1;
    }
    divisors.sort();
    divisors
}

fn main() {
    input!(n: u64, m: u64);
    let m = Mint::from_i64(m as i64);
    let divisors = divisors(n);
    let primes = primes(n);

    let only_rotations = (divisors
        .iter()
        .map(|&d| {
            let mut euler = d;
            for &p in &primes {
                if euler % p == 0 {
                    euler -= euler / p;
                }
            }
            m.pow(n / d) * Mint::from_i64(euler as i64)
        })
        .sum::<Mint>())
        / Mint::from_i64(n as i64);

    let ans = (only_rotations + only_rotations + m.pow((n + 1) / 2) + m.pow((n + 2) / 2))
        / Mint::from_i64(4);

    println!("{}", ans);
}

#[cfg(test)]
mod chap4_1_painting_stones_reflections_tests {
    const BIN: &str = "chap4_1_painting_stones_reflections";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample("2 10\n", "55\n");
    }

    #[test]
    fn sample2() {
        // 1 色: 10
        // 2 色: 90 （1 つのもの、2 つのもの）
        // 3 色: 120 (binom(10, 3))
        test_sample("3 10\n", "220\n");
    }
}
