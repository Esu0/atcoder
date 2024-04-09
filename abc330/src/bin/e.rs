use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        ix: [(usize, usize); q],
    }

    let mut bk = Vec::with_capacity(n);
    bk.resize(n, 0usize);
    a.iter().for_each(|ai| {
        if let Some(x) = bk.get_mut(*ai) {
            *x += 1;
        }
    });
    let mut s: BTreeSet<_> = bk.iter().enumerate().filter(|&(_, &x)| x == 0).map(|(i, _)| i).collect();
    for (i, x) in ix.into_iter().map(|(i, x)| (i - 1, x)) {
        let old = std::mem::replace(&mut a[i], x);
        if let Some(mref) = bk.get_mut(x) {
            if *mref == 0 {
                if !s.remove(&x) {
                    panic!()
                }
            }
            *mref += 1;
        }
        if let Some(x) = bk.get_mut(old) {
            *x -= 1;
            if *x == 0 {
                if !s.insert(old) {
                    panic!()
                }
            }
        }
        println!("{}", s.first().copied().unwrap_or(n));
    }
}
