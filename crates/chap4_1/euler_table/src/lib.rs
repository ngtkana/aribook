#![allow(clippy::many_single_char_names)]

pub fn euler_table(len: usize) -> Vec<usize> {
    let mut euler = (0..len).collect::<Vec<_>>();
    for i in 2..len {
        if euler[i] == i {
            let mut j = i;
            while j < len {
                euler[j] -= euler[j] / i;
                j += i;
            }
        }
    }
    euler
}

#[cfg(test)]
mod chap4_1_euler_table_tests {
    use super::*;

    #[test]
    fn sample1() {
        // Assume phi(0) = 0.
        let expected = [
            0, 1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4, 12, 6, 8, 8, 16, 6, 18, 8, 12, 10, 22, 8, 20,
            12, 18, 12, 28, 8, 30, 16, 20, 16, 24, 12, 36, 18, 24, 16, 40, 12, 42, 20, 24, 22, 46,
            16, 42, 20, 32, 24, 52, 18, 40, 24, 36, 28, 58, 16, 60, 30, 36, 32, 48, 20, 66, 32, 44,
        ];
        expected
            .iter()
            .zip(euler_table(expected.len()).iter())
            .enumerate()
            .for_each(|(i, (&ex, &res))| {
                println!("Finding the Euler phi of {}... ( should be {})", i, ex);
                assert_eq!(ex, res);
            });
    }
}
