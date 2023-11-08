use std::collections::HashMap;

use proconio::input;

#[derive(Clone, Debug)]
struct SparseSegTreeDim2 {
    n: u32,
    m: u32,
    l: u32,
    dat: Vec<HashMap<(u32, u32), u32>>,

}

impl SparseSegTreeDim2 {
    fn new(n: u32, m: u32) -> Self {
        let n = n.next_power_of_two();
        let m = m.next_power_of_two();
        let l = m.trailing_zeros();
        let dat = vec![HashMap::new(); (n.trailing_zeros() * l) as usize];
        Self { n, m, l, dat }
    }

    fn add_range(&mut self, xrange: (u32, u32), yrange: (u32, u32), v: u32) {
        assert!(xrange.0 < xrange.1);
        assert!(yrange.0 < yrange.1);
        assert!(xrange.1 < self.n && yrange.1 < self.m);
        let mut xr = xrange;
        let mut yr = yrange;
        let mut i = 0;
        while xr.0 < xr.1 {
            if xr.0 % 2 == 1 {
                let mut j = 0;
                while yr.0 < yr.1 {
                    if yr.0 % 2 == 1 {
                        self.dat[i * self.l as usize + j].entry((xr.0, yr.0)).and_modify(|e| *e += v).or_insert(v);
                        yr.0 += 1;
                    }
                    yr.0 >>= 1;
                    if yr.1 % 2 == 1 {
                        self.dat[i * self.l as usize + j].entry((xr.0, yr.1 - 1)).and_modify(|e| *e += v).or_insert(v);
                    }
                    yr.1 >>= 1;
                    j += 1;
                }
                xr.0 += 1;
            }
            xr.0 >>= 1;
            if xr.1 % 2 == 1 {
                let mut j = 0;
                while yr.0 < yr.1 {
                    if yr.0 % 2 == 1 {
                        self.dat[i * self.l as usize + j].entry((xr.1 - 1, yr.0)).and_modify(|e| *e += v).or_insert(v);
                        yr.0 += 1;
                    }
                    yr.0 >>= 1;
                    if yr.1 % 2 == 1 {
                        self.dat[i * self.l as usize + j].entry((xr.1 - 1, yr.1 - 1)).and_modify(|e| *e += v).or_insert(v);
                    }
                    yr.1 >>= 1;
                    j += 1;
                }
            }
            xr.1 >>= 1;
            i += 1;
        }
    }
}
fn main() {
    input! {
        n: usize,
        d: u32,
        w: u32,
        tx: [(u32, u32); n],
    }

    // (時刻, 位置)
    let mut seg_tree = SparseSegTreeDim2::new(200000 - d + 1, 200000 - w + 1);

    tx.iter().for_each(|(t, x)| {
        seg_tree.add_range((t.saturating_sub(d), *t), (x.saturating_sub(w), *x), 1);
    });

    for (i, s) in seg_tree.dat.iter().enumerate() {
        if !s.is_empty() {
            println!("{}x{}: {:?}", 1 << (i as u32 / seg_tree.l), 1 << (i as u32 % seg_tree.l), s)
        }
    }
}
