use proconio::{input, marker};
use rand::Rng;
use segtree::query;

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

        pub trait Additional<T> {
            type Ret;
            const IDENT: Self::Ret;
            fn extract(&self, slice: &[T]) -> Self::Ret;
        }

        pub trait QueryWith<T> {
            const IDENT: T;
            type A: Additional<T>;
            fn additional(&self) -> Self::A;
            fn query_with(
                &self,
                x: &T,
                y: &T,
                additional_x: <Self::A as Additional<T>>::Ret,
                additional_y: <Self::A as Additional<T>>::Ret,
            ) -> (T, <Self::A as Additional<T>>::Ret);
        }

        #[derive(Clone, Copy, Debug, Default)]
        pub struct NoAdditional;

        impl<T> Additional<T> for NoAdditional {
            type Ret = ();
            const IDENT: Self::Ret = ();
            fn extract(&self, _: &[T]) {}
        }

        // impl<Q, T> QueryWith<T> for Q
        // where
        //     Q: Query<T>,
        // {
        //     const IDENT: T = Q::IDENT;
        //     type A = NoAdditional;
        //     fn query_with(&self, x: &T, y: &T, _: (), _: ()) -> (T, ()) {
        //         (self.query(x, y), ())
        //     }
        // }

        macro_rules! impl_query_with_where_query {
            ($($t:ty),* $(,)?) => {
                $(impl<T> QueryWith<T> for $t
                where
                    $t: Query<T>,
                {
                    const IDENT: T = <$t as Query<T>>::IDENT;
                    type A = NoAdditional;
                    fn additional(&self) -> Self::A {
                        NoAdditional
                    }
                    fn query_with(&self, x: &T, y: &T, _: (), _: ()) -> (T, ()) {
                        (self.query(x, y), ())
                    }
                })*
            }
        }

        impl_query_with_where_query!(MinQuery, MaxQuery, SumQuery, ProdQuery, GcdQuery);

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

        impl<Q, T, U> QueryWith<U> for Mod<Q, T>
        where
            Mod<Q, T>: Query<U>,
        {
            const IDENT: U = <Mod<Q, T> as Query<U>>::IDENT;
            type A = NoAdditional;
            fn additional(&self) -> Self::A {
                NoAdditional
            }
            fn query_with(&self, x: &U, y: &U, _: (), _: ()) -> (U, ()) {
                (self.query(x, y), ())
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

        #[derive(Clone, Debug)]
        pub struct PolynomialQuery<T, SQ = SumQuery, PQ = ProdQuery> {
            exponents: Vec<T>,
            sum_query: SQ,
            prod_query: PQ,
        }

        impl<SQ: QueryWith<T, A = NoAdditional>, PQ: QueryWith<T, A = NoAdditional>, T>
            PolynomialQuery<T, SQ, PQ>
        {
            pub fn new(exp: T, n: usize) -> Self
            where
                SQ: Default,
                PQ: Default,
            {
                Self::with_query(exp, n, SQ::default(), PQ::default())
            }

            pub fn with_query(exp: T, n: usize, sum_query: SQ, prod_query: PQ) -> Self {
                if n == 0 {
                    Self {
                        exponents: vec![],
                        prod_query,
                        sum_query,
                    }
                } else if n == 1 {
                    Self {
                        exponents: vec![PQ::IDENT],
                        prod_query,
                        sum_query,
                    }
                } else {
                    let mut exponents = Vec::with_capacity(n);
                    exponents.extend([PQ::IDENT, exp]);
                    for _ in 2..n {
                        let tmp = prod_query
                            .query_with(exponents.last().unwrap(), &exponents[1], (), ())
                            .0;
                        exponents.push(tmp);
                    }
                    Self {
                        exponents,
                        prod_query,
                        sum_query,
                    }
                }
            }
        }

        #[derive(Clone, Copy, Debug, Default)]
        pub struct WithLen;

        impl<T> Additional<T> for WithLen {
            type Ret = usize;
            const IDENT: Self::Ret = 0;
            fn extract(&self, slice: &[T]) -> Self::Ret {
                slice.len()
            }
        }

        impl<SQ, PQ, T> QueryWith<T> for PolynomialQuery<T, SQ, PQ>
        where
            SQ: QueryWith<T, A = NoAdditional>,
            PQ: QueryWith<T, A = NoAdditional>,
        {
            const IDENT: T = SQ::IDENT;
            type A = WithLen;
            fn additional(&self) -> Self::A {
                WithLen
            }
            fn query_with(&self, x: &T, y: &T, degree_x: usize, degree_y: usize) -> (T, usize) {
                (
                    self.sum_query
                        .query_with(
                            x,
                            &self
                                .prod_query
                                .query_with(y, &self.exponents[degree_x], (), ())
                                .0,
                            (),
                            (),
                        )
                        .0,
                    degree_x + degree_y,
                )
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

    use query::{Additional, MinQuery, Query, QueryWith};

    #[derive(Clone, Debug)]
    pub struct SegTree<Q: QueryWith<T>, T> {
        tree: Box<[T]>,
        query: Q,
        additional: Q::A,
    }

    impl<T, Q: QueryWith<T>> SegTree<Q, T> {
        /// half_len != 0
        unsafe fn make_tree_ptr(
            half_len: usize,
            f: impl FnOnce(*mut T) -> usize,
            query: &Q,
            additional: &Q::A,
        ) -> *mut [T] {
            let len = half_len * 2;
            let ptr = std::alloc::alloc(Layout::array::<T>(len).unwrap()) as *mut T;
            {
                let data_ptr = ptr.add(half_len);
                let orig_len = f(data_ptr);
                for i in orig_len..half_len {
                    data_ptr.add(i).write(Q::IDENT);
                }
            }
            ptr.write(Q::IDENT);
            Self::eval(ptr, half_len, query, additional);

            std::ptr::slice_from_raw_parts_mut(ptr, len)
        }

        unsafe fn from_write_fn(
            half_len: usize,
            query: Q,
            additional: Q::A,
            f: impl FnOnce(*mut T) -> usize,
        ) -> Self {
            Self {
                tree: Box::from_raw(Self::make_tree_ptr(half_len, f, &query, &additional)),
                query,
                additional,
            }
        }

        fn new_empty(query: Q) -> Self {
            Self {
                tree: Box::new([]),
                additional: query.additional(),
                query,
            }
        }

        /// データのスライスからセグメント木を構築する。
        pub fn new(query: Q, data: &[T]) -> Self
        where
            T: Clone,
        {
            let orig_len = data.len();
            if orig_len == 0 {
                return Self::new_empty(query);
            }
            let half_len = orig_len.next_power_of_two();
            let additional = query.additional();
            unsafe {
                Self::from_write_fn(half_len, query, additional, |data_ptr| {
                    for (i, data_i) in data.iter().enumerate() {
                        data_ptr.add(i).write(data_i.clone())
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
            let additional = query.additional();
            if size_max == Some(0) {
                Self::new_empty(query)
            } else {
                assert_ne!(size_min, 0);
                let half_len_min = size_min.next_power_of_two();
                let half_len_max = size_max.map(usize::next_power_of_two);
                if Some(half_len_min) == half_len_max {
                    let half_len = half_len_min;
                    unsafe {
                        Self::from_write_fn(half_len, query, additional, move |data_ptr| {
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
                            additional,
                            move |data_ptr| {
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

        unsafe fn eval(ptr: *mut T, half_len: usize, query: &Q, additional: &Q::A) {
            let len = half_len * 2;
            let mut range_end = len;
            let mut range_len = 1;
            for i in (1..half_len).rev() {
                range_end -= range_len * 2;
                let slice1 =
                    std::ptr::slice_from_raw_parts(ptr.add(range_end + range_len), range_len);
                let slice2 = std::ptr::slice_from_raw_parts(ptr.add(range_end), range_len);
                ptr.add(i).write(
                    query
                        .query_with(
                            &*ptr.add(i * 2),
                            &*ptr.add(i * 2 + 1),
                            additional.extract(&*slice1),
                            additional.extract(&*slice2),
                        )
                        .0,
                );
                if range_end <= half_len {
                    range_end = len;
                    range_len <<= 1;
                }
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
            let mut l_query_a = Q::A::IDENT;
            let mut r_query = Q::IDENT;
            let mut r_query_a = Q::A::IDENT;

            let mut l_range_start = l;
            // let mut l_range_end = l + 1;
            // let mut r_range_start = r - 1;
            let mut r_range_end = r;
            let mut arr_len = 1usize;
            while r - l > 2 {
                if l & 1 == 1 {
                    let l_range_end = l_range_start + arr_len;
                    let ret = unsafe {
                        self.query.query_with(
                            &l_query,
                            &self.tree[l],
                            l_query_a,
                            self.additional
                                .extract(self.tree.get_unchecked(l_range_start..l_range_end)),
                        )
                    };
                    l_query = ret.0;
                    l_query_a = ret.1;
                    l += 1;
                    l_range_start = l_range_end;
                }
                if r & 1 == 1 {
                    r -= 1;
                    let r_range_start = r_range_end - arr_len;
                    let ret = unsafe {
                        self.query.query_with(
                            &self.tree[r],
                            &r_query,
                            self.additional
                                .extract(self.tree.get_unchecked(r_range_start..r_range_end)),
                            r_query_a,
                        )
                    };
                    r_query = ret.0;
                    r_query_a = ret.1;
                    r_range_end = r_range_start;
                }
                arr_len <<= 1;
                l >>= 1;
                r >>= 1;
            }
            let a = unsafe {
                self.query.query_with(
                    &l_query,
                    &self.tree[l],
                    l_query_a,
                    self.additional.extract(
                        self.tree
                            .get_unchecked(l_range_start..l_range_start + arr_len),
                    ),
                )
            };
            if r - l == 2 {
                // [&self.tree[l], &self.tree[l + 1], &r_query]
                //     .into_iter()
                //     .fold(l_query, |acc, x| self.query.query(&acc, x))
                let b = unsafe {
                    self.query.query_with(
                        &a.0,
                        &self.tree[l + 1],
                        a.1,
                        self.additional
                            .extract(self.tree.get_unchecked(r_range_end - arr_len..r_range_end)),
                    )
                };
                self.query.query_with(&b.0, &r_query, b.1, r_query_a).0
            } else {
                // [&self.tree[l], &r_query]
                //     .into_iter()
                //     .fold(l_query, |acc, x| self.query.query(&acc, x))
                self.query.query_with(&a.0, &r_query, a.1, r_query_a).0
            }
        }

        /// 指定位置の要素をO(log(n))で更新する。
        pub fn update(&mut self, i: usize, val: T) {
            let mut i = i
                .checked_add(self.len())
                .unwrap_or_else(|| panic!("attempt to index slice maximum usize"));
            self.tree[i] = val;
            let mut range_start = i;
            let mut range_len = 1;
            while i > 1 {
                i >>= 1;
                range_start &= !range_len;
                unsafe {
                    self.tree[i] = self
                        .query
                        .query_with(
                            self.tree.get_unchecked(i * 2),
                            self.tree.get_unchecked(i * 2 + 1),
                            self.additional.extract(
                                self.tree
                                    .get_unchecked(range_start..range_start + range_len),
                            ),
                            self.additional.extract(self.tree.get_unchecked(
                                range_start + range_len..range_start + range_len * 2,
                            )),
                        )
                        .0;
                }
                range_len <<= 1;
            }
        }

        /// `pred(self.query(l..j))`が`true`となる最大の`j`をO(log(n))で求める。
        pub fn partition_point<P>(&self, l: usize, mut pred: P) -> usize
        where
            P: FnMut(&T) -> bool,
            <Q::A as Additional<T>>::Ret: Clone,
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
            let mut l_range_start = l;
            let mut arr_len = 1usize;

            let mut l_query = Q::IDENT;
            let mut l_query_a = Q::A::IDENT;
            loop {
                if l & 1 == 1 {
                    let next_query = unsafe {
                        self.query.query_with(
                            &l_query,
                            &self.tree[l],
                            l_query_a.clone(),
                            self.additional.extract(
                                self.tree
                                    .get_unchecked(l_range_start..l_range_start + arr_len),
                            ),
                        )
                    };
                    let next_l = l + 1;
                    if pred(&next_query.0) {
                        if next_l.is_power_of_two() {
                            return self.len();
                        }
                    } else {
                        break;
                    }
                    l_query = next_query.0;
                    l_query_a = next_query.1;
                    l = next_l;
                    l_range_start += arr_len;
                }
                arr_len <<= 1;
                l >>= 1;
            }
            loop {
                let next_l = l << 1;
                let Some(val) = self.tree.get(next_l) else {
                    return l - self.len();
                };
                l = next_l;
                arr_len >>= 1;
                let next_query = unsafe {
                    self.query.query_with(
                        &l_query,
                        val,
                        l_query_a.clone(),
                        self.additional.extract(
                            self.tree
                                .get_unchecked(l_range_start..l_range_start + arr_len),
                        ),
                    )
                };
                if pred(&next_query.0) {
                    l_query = next_query.0;
                    l_query_a = next_query.1;
                    l += 1;
                    l_range_start += arr_len;
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
        pub fn upper_bound<Q>(&self, k: &Q) -> usize
        where
            T: Ord + Borrow<Q>,
            Q: Ord + ?Sized,
        {
            self.partition_point(0, |v| v.borrow() > k)
        }
    }

    impl<I, Q: QueryWith<I> + Default> FromIterator<I> for SegTree<Q, I> {
        fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
            Self::from_iter_query(Q::default(), iter)
        }
    }
}

#[allow(dead_code)]
mod rolling_hash {
    use std::ops::RangeBounds;

    pub struct RollingHasher {
        modulo: u64,
        /// `exponents[i] = base^(i + 1) % modulo`
        exponents: Vec<u64>,
        hash: Vec<u64>,
    }

    pub const MODULO_DEFAULT: u64 = (1 << 61) - 1;

    impl RollingHasher {
        pub fn new(base: u64, data: impl IntoIterator<Item = u64>) -> Self {
            let mut e = 1u64;
            let modulo = MODULO_DEFAULT;
            let base = base as u128;
            let mut tmp = 0u64;
            let hash = data
                .into_iter()
                .map(|x| {
                    tmp = ((tmp as u128 * base + x as u128) % modulo as u128) as u64;
                    tmp
                })
                .collect::<Vec<_>>();
            let exponents = std::iter::repeat_with(|| {
                e = (e as u128 * base % modulo as u128) as u64;
                e
            })
            .take(hash.len() - 1)
            .collect();
            Self {
                modulo,
                exponents,
                hash,
            }
        }

        pub fn hash(&self, range: impl RangeBounds<usize>) -> u64 {
            let start = match range.start_bound() {
                std::ops::Bound::Included(&x) => x,
                std::ops::Bound::Excluded(&x) => x + 1,
                std::ops::Bound::Unbounded => 0,
            };
            let end = match range.end_bound() {
                std::ops::Bound::Included(&x) => x + 1,
                std::ops::Bound::Excluded(&x) => x,
                std::ops::Bound::Unbounded => self.hash.len(),
            };
            let mut ret = self.hash[end - 1] as i128;
            let modulo = self.modulo as i128;
            if start > 0 {
                ret = (ret
                    - self.hash[start - 1] as i128 * self.exponents[end - start - 1] as i128)
                    .rem_euclid(modulo);
            }
            ret as _
        }

        pub fn exponents(&self) -> &[u64] {
            &self.exponents
        }
    }
}

impl query::QueryWith<u64> for rolling_hash::RollingHasher {
    const IDENT: u64 = 0;
    type A = query::WithLen;
    fn additional(&self) -> Self::A {
        query::WithLen
    }

    fn query_with(
        &self,
        x: &u64,
        y: &u64,
        additional_x: <Self::A as query::Additional<u64>>::Ret,
        additional_y: <Self::A as query::Additional<u64>>::Ret,
    ) -> (u64, <Self::A as query::Additional<u64>>::Ret) {
        (
            ((*x as u128
                + *y as u128
                    * if additional_x == 0 {
                        1
                    } else {
                        self.exponents()[additional_x - 1] as u128
                    })
                % rolling_hash::MODULO_DEFAULT as u128) as u64,
            additional_x + additional_y,
        )
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        s: marker::Bytes,
    }
    let mut rng = rand::thread_rng();
    let exp1 = rng.gen_range(2..rolling_hash::MODULO_DEFAULT - 1);
    // let exp2 = rng.gen_range(2..rolling_hash::MODULO_DEFAULT - 1);
    let mut st1 = segtree::SegTree::from_iter_query(
        rolling_hash::RollingHasher::new(exp1, s.iter().map(|&x| x as u64)),
        s.iter().map(|&x| x as u64),
    );
    // let mut st2 = segtree::SegTree::from_iter_query(
    //     rolling_hash::RollingHasher::new(exp2, s.iter().map(|&x| x as u64)),
    //     s.iter().map(|&x| x as u64),
    // );
    let mut st1_rev = segtree::SegTree::from_iter_query(
        rolling_hash::RollingHasher::new(exp1, s.iter().rev().map(|&x| x as u64)),
        s.iter().rev().map(|&x| x as u64),
    );
    // let mut st2_rev = segtree::SegTree::from_iter_query(
    //     rolling_hash::RollingHasher::new(exp2, s.iter().rev().map(|&x| x as u64)),
    //     s.iter().rev().map(|&x| x as u64),
    // );

    for _ in 0..q {
        input! {
            t: u8,
        }
        if t == 1 {
            input! {
                x: usize,
                c: char,
            }
            st1.update(x - 1, c as u64);
            // st2.update(x - 1, c as u64);
            st1_rev.update(n - x, c as u64);
            // st2_rev.update(n - x, c as u64);
        } else {
            input! {
                l: usize,
                r: usize,
            }
            let m1 = (l + r - 1) / 2;
            let m2 = (l + r) / 2;
            // eprintln!("{} {} {} {}", l - 1, m1, n - r, n - m2);
            if st1.query(l - 1..m1) == st1_rev.query(n - r..n - m2)
                // && st2.query(l - 1..m1) == st2_rev.query(n - r..n - m2)
            {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
