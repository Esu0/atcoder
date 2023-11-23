//! 全域木の列挙

use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: u64,
        uvw: [(usize, usize, u64); m],
    }
    let uvw: [(usize, usize, u64); 28]  = std::array::from_fn(|i| uvw.get(i).copied().unwrap_or((0, 0, 0)));
    let a = uvw[..m].iter().combinations(n - 1).filter_map(|g| {
        let mut uf = UnionFind::new(n);
        let mut sum = 0;
        for (u, v, w) in &g {
            if !uf.union(*u - 1, *v - 1) {
                return None;
            }
            sum += *w;
        }
        std::mem::forget(g);
        std::mem::forget(uf);
        Some(sum % k)
    }).min().unwrap();
    println!("{}", a);
}
