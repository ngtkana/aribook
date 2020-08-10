use proconio::{input, marker::Usize1};
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
        pub fn with_size(len: usize) -> Self {
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

#[allow(clippy::many_single_char_names)]
fn main() {
    input!(n: usize, q: usize);
    let mut uf = union_find::UnionFind::with_size(3 * n);
    let mut ans = 0;
    for _ in 0..q {
        input!(s: String);
        match s.as_ref() {
            "same" => {
                input!(u: Usize1, v: Usize1);
                if n <= u || n <= v || uf.same(u, v + n) || uf.same(u, v + 2 * n) {
                    ans += 1;
                } else {
                    for d in 0..3 {
                        uf.unite(u + d * n, v + d * n);
                    }
                }
            }
            "eat" => {
                input!(u: Usize1, v: Usize1);
                if n <= u || n <= v || uf.same(u, v) || uf.same(u, v + 2 * n) {
                    ans += 1;
                } else {
                    for d in 0..3 {
                        uf.unite(u + d * n, v + (d + 1) % 3 * n);
                    }
                }
            }
            _ => panic!(),
        }
    }
    println!("{}", ans);
}

#[cfg(test)]
mod chap2_4_food_chain_tests {
    const BIN: &str = "chap2_4_food_chain";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"100 7
same 101 1
eat 1 2
eat 2 3
eat 3 3
same 1 3
eat 3 1
same 5 5
"#,
            "3\n",
        );
    }
}
