#![allow(clippy::many_single_char_names)]
// union_find {{{
#[allow(dead_code)]
mod union_find {
    #[derive(Clone, Copy, Debug)]
    enum ParentOrSize {
        Parent(usize),
        Size(usize),
    }

    #[derive(Clone, Debug)]
    pub struct UnionFind(Vec<ParentOrSize>);

    impl UnionFind {
        pub fn with_len(len: usize) -> Self {
            Self(vec![ParentOrSize::Size(1); len])
        }

        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        pub fn len(&self) -> usize {
            self.0.len()
        }

        pub fn find(&mut self, i: usize) -> usize {
            self.find_and_size(i).0
        }

        pub fn size(&mut self, i: usize) -> usize {
            self.find_and_size(i).1
        }

        fn find_and_size(&mut self, i: usize) -> (usize, usize) {
            assert!(i < self.len());
            match self.0[i] {
                ParentOrSize::Parent(p) => {
                    let ret = self.find_and_size(p);
                    self.0[i] = ParentOrSize::Parent(ret.0);
                    ret
                }
                ParentOrSize::Size(si) => (i, si),
            }
        }

        pub fn unite(&mut self, u: usize, v: usize) {
            let (mut u, su) = self.find_and_size(u);
            let (mut v, sv) = self.find_and_size(v);
            if u == v {
                return;
            }
            if su > sv {
                std::mem::swap(&mut u, &mut v);
                std::mem::swap(&mut v, &mut u);
            }
            self.0[v] = ParentOrSize::Size(su + sv);
            self.0[u] = ParentOrSize::Parent(v);
        }

        pub fn same(&mut self, u: usize, v: usize) -> bool {
            self.find(u) == self.find(v)
        }
    }
}
// }}}
use proconio::input;

fn main() {
    input!(a: u64, b: u64, p_min: u64);
    let p_max = b.min(100_000);
    let mut sieve = vec![false; p_max as usize + 1];
    for p in 2..=p_max {
        for i in (2..).take_while(|&i| i * p <= p_max) {
            sieve[i as usize * p as usize] = true;
        }
    }
    let mut uf = union_find::UnionFind::with_len((b - a + 1) as usize);
    for p in (p_min..=p_max).filter(|&i| !sieve[i as usize]) {
        let start = (a + p - 1) / p * p;
        for i in std::iter::successors(Some(start + p), |x| Some(x + p)).take_while(|&i| i <= b) {
            uf.unite((start - a) as usize, (i - a) as usize);
        }
    }
    println!(
        "{}",
        (0..(b - a + 1) as usize)
            .filter(|&i| uf.find(i) == i)
            .count()
    );
}

#[cfg(test)]
mod chap3_7_number_sets_tests {
    const BIN: &str = "chap3_7_number_sets";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample("10 20 5\n", "9\n");
    }

    #[test]
    fn sample2() {
        test_sample("10 20 3\n", "7\n");
    }
}
