use std::str::FromStr;

use proconio::input;

#[allow(dead_code)]
mod segtree {
    pub mod query {
        use std::{
            cmp::Ordering,
            ops::{Add, Mul, Rem},
        };

        use self::ident::{HasAddIdent, HasMax, HasMin, HasMulIdent};

        pub mod ident {
            use std::cmp::Reverse;

            pub(super) trait HasAddIdent {
                const IDENT: Self;
            }

            pub(super) trait HasMulIdent {
                const IDENT: Self;
            }

            macro_rules! has_add_ident_num_impl {
                ($t:ty) => {
                    impl HasAddIdent for $t {
                        const IDENT: Self = 0;
                    }
                };
            }

            macro_rules! has_mul_ident_num_impl {
                ($t:ty) => {
                    impl HasMulIdent for $t {
                        const IDENT: Self = 1;
                    }
                };
            }

            has_add_ident_num_impl! {u8}
            has_add_ident_num_impl! {u16}
            has_add_ident_num_impl! {u32}
            has_add_ident_num_impl! {u64}
            has_add_ident_num_impl! {u128}
            has_add_ident_num_impl! {i8}
            has_add_ident_num_impl! {i16}
            has_add_ident_num_impl! {i32}
            has_add_ident_num_impl! {i64}
            has_add_ident_num_impl! {i128}

            has_mul_ident_num_impl! {u8}
            has_mul_ident_num_impl! {u16}
            has_mul_ident_num_impl! {u32}
            has_mul_ident_num_impl! {u64}
            has_mul_ident_num_impl! {u128}
            has_mul_ident_num_impl! {i8}
            has_mul_ident_num_impl! {i16}
            has_mul_ident_num_impl! {i32}
            has_mul_ident_num_impl! {i64}
            has_mul_ident_num_impl! {i128}

            pub(super) trait HasMin {
                const MIN: Self;
            }

            macro_rules! has_min_num_impl {
                ($t:ty) => {
                    impl HasMin for $t {
                        const MIN: Self = <$t>::MIN;
                    }
                };
            }

            has_min_num_impl! {u8}
            has_min_num_impl! {u16}
            has_min_num_impl! {u32}
            has_min_num_impl! {u64}
            has_min_num_impl! {u128}
            has_min_num_impl! {i8}
            has_min_num_impl! {i16}
            has_min_num_impl! {i32}
            has_min_num_impl! {i64}
            has_min_num_impl! {i128}

            pub(super) trait HasMax {
                const MAX: Self;
            }

            macro_rules! has_max_num_impl {
                ($t:ty) => {
                    impl HasMax for $t {
                        const MAX: Self = <$t>::MAX;
                    }
                };
            }

            has_max_num_impl! {u8}
            has_max_num_impl! {u16}
            has_max_num_impl! {u32}
            has_max_num_impl! {u64}
            has_max_num_impl! {u128}
            has_max_num_impl! {i8}
            has_max_num_impl! {i16}
            has_max_num_impl! {i32}
            has_max_num_impl! {i64}
            has_max_num_impl! {i128}

            impl<T: HasMax> HasMin for Reverse<T> {
                const MIN: Self = Self(<T as HasMax>::MAX);
            }

            impl<T: HasMin> HasMax for Reverse<T> {
                const MAX: Self = Self(<T as HasMin>::MIN);
            }
        }

        pub trait Query<T> {
            const IDENT: T;
            fn query(&self, x: &T, y: &T) -> T;
        }

        #[derive(Clone, Copy, Debug, Default)]
        pub struct MinQuery;

        impl<T: Ord + HasMax + Clone> Query<T> for MinQuery {
            const IDENT: T = T::MAX;
            fn query(&self, x: &T, y: &T) -> T {
                match x.cmp(y) {
                    Ordering::Less => x.clone(),
                    _ => y.clone(),
                }
            }
        }

        #[derive(Clone, Copy, Debug, Default)]
        pub struct MaxQuery;

        impl<T: Ord + HasMin + Clone> Query<T> for MaxQuery {
            const IDENT: T = T::MIN;
            fn query(&self, x: &T, y: &T) -> T {
                match x.cmp(y) {
                    Ordering::Less => y.clone(),
                    _ => x.clone(),
                }
            }
        }

        #[derive(Clone, Copy, Debug, Default)]
        pub struct SumQuery;

        impl<T: Add<Output = T> + HasAddIdent + Clone> Query<T> for SumQuery {
            const IDENT: T = T::IDENT;
            fn query(&self, x: &T, y: &T) -> T {
                x.clone() + y.clone()
            }
        }

        #[derive(Clone, Copy, Debug, Default)]
        pub struct ProdQuery;

        impl<T: Mul<Output = T> + HasMulIdent + Clone> Query<T> for ProdQuery {
            const IDENT: T = T::IDENT;
            fn query(&self, x: &T, y: &T) -> T {
                x.clone() * y.clone()
            }
        }

        #[derive(Clone, Copy, Debug)]
        pub struct Mod<Q, T> {
            base: Q,
            modulo: T,
        }

        impl<Q, T: Rem<Output = T>> Mod<Q, T> {
            pub fn new(query: Q, modulo: T) -> Self {
                Self {
                    base: query,
                    modulo,
                }
            }
        }

        impl<Q, T, U> Query<U> for Mod<Q, T>
        where
            Q: Query<U>,
            T: Clone,
            U: Clone + Rem<T, Output = U>,
        {
            const IDENT: U = Q::IDENT;
            fn query(&self, x: &U, y: &U) -> U {
                self.base.query(x, y) % self.modulo.clone()
            }
        }

        #[derive(Clone, Copy, Debug, Default)]
        pub struct GcdQuery;

        impl<T: Rem<Output = T> + HasAddIdent + Eq + Clone> Query<T> for GcdQuery {
            const IDENT: T = T::IDENT;
            fn query(&self, x: &T, y: &T) -> T {
                if x == &T::IDENT {
                    return y.clone();
                }
                let mut x = x.clone();
                let mut y = y.clone();
                while y != T::IDENT {
                    let tmp = x.clone() % y.clone();
                    x = std::mem::replace(&mut y, tmp);
                }
                x
            }
        }
    }

    use std::{
        alloc::Layout,
        borrow::Borrow,
        cmp::Ordering,
        mem::MaybeUninit,
        ops::{Bound, RangeBounds},
    };

    use query::{MinQuery, Query};

    #[derive(Clone, Debug)]
    pub struct SegTree<Q, T> {
        tree: Box<[T]>,
        query: Q,
    }

    impl<T, Q: Query<T>> SegTree<Q, T> {
        /// half_len != 0
        unsafe fn make_tree_ptr(
            half_len: usize,
            f: impl FnOnce(*mut T, &Q) -> usize,
            query: &Q,
        ) -> *mut [T] {
            let len = half_len * 2;
            let ptr = std::alloc::alloc(Layout::array::<T>(len).unwrap()) as *mut T;
            {
                let data_ptr = ptr.add(half_len);
                let orig_len = f(data_ptr, query);
                for i in orig_len..half_len {
                    data_ptr.add(i).write(Q::IDENT);
                }
            }
            ptr.write(Q::IDENT);
            Self::eval(ptr, half_len, query);

            std::ptr::slice_from_raw_parts_mut(ptr, len)
        }

        unsafe fn from_write_fn(
            half_len: usize,
            query: Q,
            f: impl FnOnce(*mut T, &Q) -> usize,
        ) -> Self {
            Self {
                tree: Box::from_raw(Self::make_tree_ptr(half_len, f, &query)),
                query,
            }
        }

        fn new_empty(query: Q) -> Self {
            Self {
                tree: Box::new([]),
                query,
            }
        }

        /// データのスライスからセグメント木を構築する。
        pub fn new(query: Q, data: &[T]) -> Self {
            let orig_len = data.len();
            if orig_len == 0 {
                return Self::new_empty(query);
            }
            let half_len = orig_len.next_power_of_two();
            unsafe {
                Self::from_write_fn(half_len, query, |data_ptr, query| {
                    for (i, data_i) in data.iter().enumerate() {
                        data_ptr.add(i).write(query.query(data_i, &Q::IDENT))
                    }
                    orig_len
                })
            }
        }

        pub fn from_iter_query<I>(query: Q, iter: I) -> Self
        where
            I: IntoIterator<Item = T>,
        {
            let iter = iter.into_iter();
            let (size_min, size_max) = iter.size_hint();
            if size_max == Some(0) {
                Self::new_empty(query)
            } else {
                assert_ne!(size_min, 0);
                let half_len_min = size_min.next_power_of_two();
                let half_len_max = size_max.map(usize::next_power_of_two);
                if Some(half_len_min) == half_len_max {
                    let half_len = half_len_min;
                    unsafe {
                        Self::from_write_fn(half_len, query, move |data_ptr, _| {
                            let mut i = 0;
                            for item in iter {
                                data_ptr.add(i).write(item);
                                i += 1;
                            }
                            i
                        })
                    }
                } else {
                    let mut data = iter.collect::<Vec<_>>();
                    let orig_len = data.len();
                    unsafe {
                        Self::from_write_fn(
                            orig_len.next_power_of_two(),
                            query,
                            move |data_ptr, _| {
                                let src = data.as_mut_ptr();
                                let cap = data.capacity();
                                std::mem::forget(data);
                                data_ptr.copy_from_nonoverlapping(src, orig_len);
                                // `I`のデストラクタは呼ばずにメモリの解放のみ行う
                                drop(Vec::from_raw_parts(
                                    src as *mut MaybeUninit<I>,
                                    orig_len,
                                    cap,
                                ));
                                orig_len
                            },
                        )
                    }
                }
            }
        }

        pub fn len(&self) -> usize {
            self.tree.len() / 2
        }

        pub fn is_empty(&self) -> bool {
            self.tree.is_empty()
        }

        unsafe fn eval(ptr: *mut T, half_len: usize, query: &Q) {
            for i in (1..half_len).rev() {
                ptr.add(i)
                    .write(query.query(&*ptr.add(i * 2), &*ptr.add(i * 2 + 1)));
            }
        }

        /// 戻り値を`(l, r)`とすると以下が保証される。
        ///
        /// * `l <= r <= self.len()`
        fn get_lr(&self, range: impl RangeBounds<usize>) -> (usize, usize) {
            let size = self.len();
            let l = match range.start_bound() {
                Bound::Excluded(s) => s
                    .checked_add(1)
                    .unwrap_or_else(|| panic!("attempted to index slice from after maximum usize")),
                Bound::Included(s) => *s,
                Bound::Unbounded => 0,
            };
            let r = match range.end_bound() {
                Bound::Excluded(e) => *e,
                Bound::Included(e) => e
                    .checked_add(1)
                    .unwrap_or_else(|| panic!("attempted to index slice up to maximum usize")),
                Bound::Unbounded => size,
            };
            if l > r {
                panic!("slice index starts at {l} but ends at {r}");
            } else if r > size {
                panic!("range end index {r} out of range for slice of length {size}");
            }
            (l, r)
        }

        /// 指定区間のクエリをO(log(n))で求める。
        pub fn query(&self, range: impl RangeBounds<usize>) -> T {
            let (mut l, mut r) = self.get_lr(range);
            if r == l {
                return Q::IDENT;
            }
            l += self.len();
            r += self.len();
            let mut l_query = Q::IDENT;
            let mut r_query = Q::IDENT;
            while r - l > 2 {
                if l & 1 == 1 {
                    l_query = self.query.query(&l_query, &self.tree[l]);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    r_query = self.query.query(&self.tree[r], &r_query);
                }
                l >>= 1;
                r >>= 1;
            }
            if r - l == 2 {
                [&self.tree[l], &self.tree[l + 1], &r_query]
                    .into_iter()
                    .fold(l_query, |acc, x| self.query.query(&acc, x))
            } else {
                [&self.tree[l], &r_query]
                    .into_iter()
                    .fold(l_query, |acc, x| self.query.query(&acc, x))
            }
        }

        /// 指定位置の要素をO(log(n))で更新する。
        pub fn update(&mut self, i: usize, val: T) {
            let mut i = i
                .checked_add(self.len())
                .unwrap_or_else(|| panic!("attempt to index slice maximum usize"));
            self.tree[i] = val;
            while i > 1 {
                i >>= 1;
                self.tree[i] = self.query.query(&self.tree[i * 2], &self.tree[i * 2 + 1]);
            }
        }

        /// `pred(self.query(l..j))`が`true`となる最大の`j`をO(log(n))で求める。
        pub fn partition_point<P>(&self, l: usize, mut pred: P) -> usize
        where
            P: FnMut(&T) -> bool,
        {
            match l.cmp(&self.len()) {
                Ordering::Equal => return l,
                Ordering::Greater => {
                    panic!("index {l} out of range for slice of length {}", self.len())
                }
                _ => {}
            }
            let mut l = l
                .checked_add(self.len())
                .unwrap_or_else(|| panic!("attempt to index maximum usize"));
            let mut l_query = Q::IDENT;
            loop {
                if l & 1 == 1 {
                    let next_query = self.query.query(&l_query, &self.tree[l]);
                    let next_l = l + 1;
                    if pred(&next_query) {
                        if next_l.is_power_of_two() {
                            return self.len();
                        } else {
                            l_query = next_query;
                        }
                    } else {
                        break;
                    }
                    l = next_l;
                }
                l >>= 1;
            }
            loop {
                let next_l = l << 1;
                let Some(val) = self.tree.get(next_l) else {
                    return l - self.len();
                };
                l = next_l;
                let next_query = self.query.query(&l_query, val);
                if pred(&next_query) {
                    l_query = next_query;
                    l += 1;
                }
            }
        }
    }

    impl<T> SegTree<MinQuery, T>
    where
        MinQuery: Query<T>,
    {
        // セグメント木の全要素を'val'で埋める。
        pub fn fill(&mut self, val: T)
        where
            T: Clone,
        {
            self.tree.fill(val);
        }

        /// セグメント木の全要素を`f`の戻り値で埋める。
        ///
        /// # Note
        /// 内部可変性を用いると`f`の戻り値が変化するようにすることもできるが、その場合はセグメント木は意味のない値で埋められ、
        /// クエリの結果も意味のない値になる。
        pub fn fill_with(&mut self, f: impl Fn() -> T) {
            self.tree.fill_with(f)
        }

        /// `k`と等しいかそれより小さい要素の最左位置をO(log(n))で求める。
        ///
        /// self.query(..j) > kとなる最大のjを返す。
        pub fn upper_bound<Q: ?Sized>(&self, k: &Q) -> usize
        where
            T: Ord + Borrow<Q>,
            Q: Ord,
        {
            self.partition_point(0, |v| v.borrow() > k)
        }
    }

    impl<I, Q: Query<I> + Default> FromIterator<I> for SegTree<Q, I> {
        fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
            Self::from_iter_query(Q::default(), iter)
        }
    }
}
enum CharOrNum {
    Char(u8),
    Num(usize),
}

impl FromStr for CharOrNum {
    type Err = <usize as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first = s.as_bytes()[0];
        if first.is_ascii_alphabetic() {
            Ok(Self::Char(first))
        } else {
            s.parse::<usize>().map(CharOrNum::Num)
        }
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        s: proconio::marker::Bytes,
        queries: [(u8, usize, CharOrNum); q],
    }
    
}
