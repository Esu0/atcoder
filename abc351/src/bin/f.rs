use std::cmp::Reverse;

use proconio::input;

fn solve(a: &[i64], buf: &mut [i64]) -> i64 {
    let mid = a.len() / 2;
    if mid == 0 {
        0
    } else {
        let (l, r) = a.split_at(mid);
        buf.copy_from_slice(a);
        let (buf_l, buf_r) = buf.split_at_mut(mid);
        buf_l.sort_unstable_by_key(|x| Reverse(*x));
        buf_r.sort_unstable_by_key(|x| Reverse(*x));
        let mut j = 0;
        let mut sum = 0;
        let mut part_ans = 0;
        for ai in &*buf_l {
            while buf_r.get(j).is_some() {
                let aj = buf_r[j];
                if aj > *ai {
                    sum += aj;
                    j += 1;
                } else {
                    break;
                }
            }
            part_ans += sum - ai * j as i64;
        }

        solve(l, buf_l) + solve(r, buf_r) + part_ans
    }
}

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut buf = vec![0; n];
    println!("{}", solve(&a, &mut buf));
}
