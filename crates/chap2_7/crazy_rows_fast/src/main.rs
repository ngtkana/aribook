// segtree {{{
#[allow(dead_code)]
mod segtree {
    pub trait SegValue: Clone + std::fmt::Debug {}

    impl<Value: Clone + std::fmt::Debug> SegValue for Value {}

    pub trait SegInfo {
        type Value: SegValue;

        fn id() -> Self::Value;

        fn op(x: &Self::Value, y: &Self::Value) -> Self::Value;

        fn op_assign(x: &mut Self::Value, y: &Self::Value) {
            *x = Self::op(x, y);
        }
    }

    pub trait Zero {
        fn zero() -> Self;
    }
    macro_rules! impl_zero {
        ($($ty: ty)+) => { $(impl Zero for $ty { fn zero() -> Self { 0 } })+ };
    }
    impl_zero! {
        u8 u16 u32 u64 u128 usize
        i8 i16 i32 i64 i128 isize
    }
    pub trait MinValue {
        fn min_value() -> Self;
    }
    macro_rules! impl_min {
        ($($ty: ident)+) => { $(impl MinValue for $ty { fn min_value() -> Self { std::$ty::MIN } })+ };
    }
    impl_min! {
        u8 u16 u32 u64 u128 usize
        i8 i16 i32 i64 i128 isize
    }

    pub trait MaxValue {
        fn max_value() -> Self;
    }
    macro_rules! impl_max {
        ($($ty: ident)+) => { $(impl MaxValue for $ty { fn max_value() -> Self { std::$ty::MAX } })+ };
    }
    impl_max! {
        u8 u16 u32 u64 u128 usize
        i8 i16 i32 i64 i128 isize
    }

    pub struct AddInfo<Value: SegValue + std::ops::Add> {
        _marker: std::marker::PhantomData<Value>,
    }

    impl<Value: SegValue + Zero + std::ops::Add<Output = Value> + std::ops::AddAssign> SegInfo
        for AddInfo<Value>
    {
        type Value = Value;
        fn id() -> Self::Value {
            Self::Value::zero()
        }
        fn op(x: &Self::Value, y: &Self::Value) -> Self::Value {
            x.clone() + y.clone()
        }
        fn op_assign(x: &mut Self::Value, y: &Self::Value) {
            *x += y.clone()
        }
    }
    pub struct MinInfo<Value: SegValue + MaxValue + std::cmp::Ord> {
        _marker: std::marker::PhantomData<Value>,
    }
    impl<Value: SegValue + MaxValue + std::cmp::Ord> SegInfo for MinInfo<Value> {
        type Value = Value;
        fn id() -> Self::Value {
            Self::Value::max_value()
        }
        fn op(x: &Self::Value, y: &Self::Value) -> Self::Value {
            x.clone().min(y.clone())
        }
    }
    pub struct MaxInfo<Value: SegValue + MinValue + std::cmp::Ord> {
        _marker: std::marker::PhantomData<Value>,
    }
    impl<Value: SegValue + MinValue + std::cmp::Ord> SegInfo for MaxInfo<Value> {
        type Value = Value;
        fn id() -> Self::Value {
            Self::Value::min_value()
        }
        fn op(x: &Self::Value, y: &Self::Value) -> Self::Value {
            x.clone().max(y.clone())
        }
    }

    #[derive(Clone)]
    pub struct Segtree<Info: SegInfo>(Vec<Info::Value>);

    impl<Info: SegInfo> Segtree<Info> {
        pub fn new(len: usize) -> Self {
            Self(vec![Info::id(); 2 * len])
        }

        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        pub fn len(&self) -> usize {
            self.0.len() / 2
        }

        pub fn as_slice(&self) -> &[Info::Value] {
            &self.0[self.len()..self.0.len()]
        }

        pub fn iter(&self) -> std::slice::Iter<Info::Value> {
            self.0[self.len()..self.0.len()].iter()
        }

        pub fn vec(&self) -> Vec<Info::Value> {
            self.0[self.len()..self.0.len()].into()
        }

        pub fn set(&mut self, i: usize, x: Info::Value) {
            let i = i + self.len();
            self.0[i] = x;
            self.rebuild_one_leaf(i);
        }

        pub fn modify(&mut self, i: usize, f: impl Fn(&mut Info::Value)) {
            let i = i + self.len();
            f(&mut self.0[i]);
            self.rebuild_one_leaf(i);
        }

        pub fn fold(&self, mut l: usize, mut r: usize) -> Info::Value {
            assert!(l <= r && r <= self.len());
            l += self.len();
            r += self.len();
            let mut fl = Info::id();
            let mut fr = Info::id();
            while l < r {
                if l % 2 == 1 {
                    Info::op_assign(&mut fl, &self.0[l]);
                    l += 1;
                }
                if r % 2 == 1 {
                    r -= 1;
                    Info::op_assign(&mut fr, &self.0[r]);
                }
                l /= 2;
                r /= 2;
            }
            Info::op(&fl, &fr)
        }

        pub fn fold_all(&self) -> Info::Value {
            self.fold(0, self.len())
        }

        fn rebuild_one_leaf(&mut self, mut i: usize) {
            i /= 2;
            while 0 != i {
                self.0[i] = Info::op(&self.0[2 * i], &self.0[2 * i + 1]);
                i /= 2
            }
        }

        fn rebuild(&mut self) {
            for i in (1..self.len()).rev() {
                self.0[i] = Info::op(&self.0[2 * i], &self.0[2 * i + 1]);
            }
        }
    }

    impl<Info: SegInfo> std::convert::From<Vec<Info::Value>> for Segtree<Info> {
        fn from(vec: Vec<Info::Value>) -> Self {
            let mut vec1 = Vec::with_capacity(vec.len() * 2);
            vec1.resize(vec.len(), Info::id());
            vec1.extend(vec);
            let mut seg = Self(vec1);
            seg.rebuild();
            seg
        }
    }

    impl<Info: SegInfo, I: std::slice::SliceIndex<[Info::Value]>> std::ops::Index<I> for Segtree<Info> {
        type Output = I::Output;

        #[inline]
        fn index(&self, index: I) -> &Self::Output {
            std::ops::Index::index(self.as_slice(), index)
        }
    }

    impl<Info: SegInfo> std::fmt::Debug for Segtree<Info> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(self.as_slice()).finish()
        }
    }

    #[macro_export]
    macro_rules! segtree {
        ($($elem:expr),*$(,)?) => {
            Segtree::from(vec![$($elem),*])
        };
        ($elem:expr; $n:expr) => {
            Segtree::from(vec![$elem; $n])
        };
    }
}
// }}}
use proconio::input;
use segtree::{AddInfo, Segtree};
use std::cmp::Reverse;

fn main() {
    input!(n: usize, g: [proconio::marker::Chars; n]);
    let mut a = vec![Vec::<Reverse<usize>>::new(); n];
    for (i, x) in g
        .iter()
        .map(|s| (0..n).rfind(|&j| s[j] == '1').map(|j| j).unwrap_or(0))
        .enumerate()
    {
        a[x].push(Reverse(i));
    }
    let mut heap = std::collections::BinaryHeap::<Reverse<usize>>::new();
    let mut seg: Segtree<AddInfo<usize>> = segtree![0; n];
    let mut ans = 0;
    for v in &a {
        heap.extend(v);
        let Reverse(j) = heap.pop().unwrap();
        ans += seg.fold(j, n);
        seg.set(j, 1);
    }
    println!("{}", ans);
}

#[cfg(test)]
mod chap2_7_crazy_rows_fast_tests {
    const BIN: &str = "chap2_7_crazy_rows_fast";

    fn test_sample(input: &str, output: &str) {
        proconcli::test_sample(input, output, BIN);
    }

    #[test]
    fn sample1() {
        test_sample(
            r#"2
10
11
"#,
            "0\n",
        );
    }

    #[test]
    fn sample2() {
        test_sample(
            r#"3
001
100
010
"#,
            "2\n",
        );
    }

    #[test]
    fn sample3() {
        test_sample(
            r#"4
1110
1100
1100
1000
"#,
            "4\n",
        );
    }
}
