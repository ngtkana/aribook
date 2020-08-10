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

pub fn kruskal(len: usize, edges: &mut [(u32, usize, usize)]) -> u32 {
    let mut uf = union_find::UnionFind::with_len(len);
    edges.sort();
    let mut ans = 0;
    for &mut (w, u, v) in edges {
        if !uf.same(u, v) {
            uf.unite(u, v);
            ans += w;
        }
    }
    ans
}

#[cfg(test)]
mod chap2_5_kruskal_tests {
    use super::*;

    #[test]
    fn test_wikipedia() {
        let mut edges = [
            (7, 0, 1),
            (5, 0, 3),
            (8, 1, 2),
            (9, 1, 3),
            (7, 1, 4),
            (5, 2, 4),
            (15, 3, 4),
            (6, 3, 5),
            (8, 4, 5),
            (9, 4, 6),
            (11, 5, 6),
        ];
        assert_eq!(kruskal(7, &mut edges), 39);
    }
}
