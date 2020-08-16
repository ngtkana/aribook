#![allow(clippy::many_single_char_names)]
use proconio::input;

fn gcd(x: u128, y: u128) -> u128 {
    if x == 0 {
        y
    } else {
        gcd(y % x, x)
    }
}

fn transpose(a: &[Vec<u128>]) -> Vec<Vec<u128>> {
    (0..a[0].len())
        .map(|j| a.iter().map(|v| v[j]).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn main() {
    input!(n: u128, t: usize);
    let d = (0..t)
        .map(|_| {
            input!(k: usize, d: [proconio::marker::Usize1; k]);
            d.iter()
                .filter(|&&d| (d as u128) < n)
                .copied()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let d_max = d.iter().flatten().copied().max().unwrap();

    let mut cum = vec![vec![0u128; d_max + 1]; t];
    for (cum, d) in cum.iter_mut().zip(d.iter()) {
        for &x in d {
            cum[x] += 1;
        }
        (0..cum.len() - 1).for_each(|i| cum[i + 1] += cum[i]);
    }
    let cum = transpose(&cum);

    let score_seq = cum
        .iter()
        .map(|row| {
            let mut ans = row.iter().sum::<u128>() * n;
            for i in 0..t {
                for j in i + 1..t {
                    ans += 2 * row[i] * row[j];
                }
            }
            ans
        })
        .collect::<Vec<_>>();

    let ans = score_seq.iter().sum::<u128>() + score_seq.last().unwrap() * (n - 1 - d_max as u128);

    let mut den = n * n;
    let int = ans / den;
    let mut num = ans % den;
    let g = gcd(num, den);
    num /= g;
    den /= g;
    println!("{}+{}/{}", int, num, den);
}

#[cfg(test)]
mod chap4_8_year_of_more_code_jam_tests {
    const BIN: &str = "chap4_8_year_of_more_code_jam";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"1 1
2 1 2
"#,
            "1+0/1\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"4 2
3 1 2 4
2 1 3
"#,
            "5+1/8\n",
        );
    }
}
