use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Bytes,
        lr: [(usize, usize); q],
    }

    // size: n
    let p: Vec<_> = std::iter::once(0).chain(s.windows(2).map(|w| (w[0] == w[1]) as u32)).collect();
    
    let n = n.next_power_of_two() * 2;
    let mut seg = vec![0; n];
    for (pi, segi) in p.iter().zip(&mut seg[n / 2..]) {
        *segi = *pi;
    }
    for i in (1..n / 2).rev() {
        seg[i] = seg[i * 2] + seg[i * 2 + 1];
    }
    // println!("{:?}", p);
    // println!("{:?}", seg);

    for (l, r) in lr {
        let mut l = l + n / 2;
        let mut r = r + n / 2;
        let mut sum = 0;
        while l < r {
            if l % 2 == 1 {
                sum += seg[l];
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                sum += seg[r];
            }
            l /= 2;
            r /= 2;
        }
        println!("{sum}");
    }
}
