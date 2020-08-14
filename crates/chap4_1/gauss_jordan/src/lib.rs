#![allow(clippy::many_single_char_names)]
use ordered_float::NotNan;
const EPS: f64 = 1e-8;
pub fn solve_non_singular_linear_eq_gauss_jordan(a: &[Vec<f64>], b: &[f64]) -> Option<Vec<f64>> {
    assert_eq!(a.len(), b.len());
    let n = a.len();
    let mut a = a
        .iter()
        .zip(b.iter())
        .map(|(v, y)| {
            v.iter()
                .chain(std::iter::once(y))
                .copied()
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<_>>();
    for j in 0..n {
        let pivot_i = (j..n)
            .max_by_key(|&pivot| NotNan::new(a[pivot][j]).unwrap())
            .unwrap();
        a.swap(j, pivot_i);
        if (a[j][j]).abs() < EPS {
            return None;
        }
        let recip = a[j][j].recip();
        a[j][j] = 1.0;
        a[j][j + 1..].iter_mut().for_each(|x| *x *= recip);
        let (left, center_right) = a.split_at_mut(j);
        let (center, right) = center_right.split_at_mut(1);
        let aj = center.get_mut(0).unwrap();
        for v in left.iter_mut().chain(right.iter_mut()) {
            let r = v[j];
            v[j] = 0.0;
            for (x, y) in aj[j + 1..].iter().zip(v[j + 1..].iter_mut()) {
                *y -= x * r;
            }
        }
    }
    Some(a.iter().map(|v| *v.last().unwrap()).collect())
}

#[cfg(test)]
mod chap4_1_gauss_jordan_tests {
    use super::*;

    #[test]
    fn editorial() {
        let a = [
            vec![1.0, -2.0, 3.0],
            vec![4.0, -5.0, 6.0],
            vec![7.0, -8.0, 10.0],
        ];
        let b = [6.0, 12.0, 21.0];
        let c = solve_non_singular_linear_eq_gauss_jordan(&a, &b).unwrap();
        let expected = [1.0, 2.0, 3.0];
        println!("c = {:?}", &c);
        println!("expected = {:?}", &expected);
        for (&x, &y) in c.iter().zip(expected.iter()) {
            assert!((x - y).abs() < EPS * (x.abs() + y.abs()));
        }
    }
}
