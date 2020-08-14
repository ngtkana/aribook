#![allow(clippy::many_single_char_names)]

pub fn mobius_of_divisors(mut n: u64) -> Vec<(u64, i32)> {
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
    (0..1 << primes.len())
        .map(|bs: u32| {
            (
                primes
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &x)| if bs >> i & 1 == 0 { None } else { Some(x) })
                    .product::<u64>(),
                match bs.count_ones() % 2 {
                    0 => 1,
                    1 => -1,
                    _ => unreachable!(),
                },
            )
        })
        .collect()
}

#[cfg(test)]
mod chap4_1_mobius_function_of_divisors_tests {
    use super::*;

    #[test]
    fn test_hand() {
        let n = 60;
        let mut a = mobius_of_divisors(n);
        a.sort();
        assert_eq!(
            a,
            vec![
                (1, 1),
                (2, -1),
                (3, -1),
                (5, -1),
                (6, 1),
                (10, 1),
                (15, 1),
                (30, -1)
            ]
        );
    }
}
