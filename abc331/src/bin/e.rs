use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        a: [i64; n],
        b: [i64; m],
        mut cd: [(usize, usize); l],
    }

    cd.sort_unstable_by_key(|(ci, di)| (*ci, Reverse(b[*di - 1]), *di));
    // println!("{cd:?}");
    let mut b = b
        .into_iter()
        .enumerate()
        .map(|(i, bi)| (i + 1, bi))
        .collect::<Vec<_>>();
    b.sort_by_key(|(_, bi)| Reverse(*bi));
    // println!("{b:?}");

    let mut j = 0;
    let ans = a
        .iter()
        .enumerate()
        .map(|(i, ai)| (i + 1, ai))
        .filter_map(|(i, &ai)| {
            let mut k = 0;
            while j < l && i == cd[j].0 && k < m && cd[j].1 == b[k].0 {
                // 提供されていない定食
                j += 1;
                k += 1;
            }
            while j < l && cd[j].0 <= i {
                j += 1;
            }
            b.get(k).map(|(_, bk)| ai + *bk)
        })
        .max()
        .unwrap();

    println!("{ans}");
}
