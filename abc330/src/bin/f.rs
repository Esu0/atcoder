use std::cmp::Ordering;

use proconio::input;

fn cost(p: &[i64], s: i64) -> u64 {
    let mut v = Vec::with_capacity(p.len() * 2);
    let mut p1 = p.iter().copied().peekable();
    let mut p2 = p.iter().map(|x| x - s as i64).peekable();
    while let Some(x1) = p1.peek() {
        if let Some(x2) = p2.peek() {
            match x1.cmp(x2) {
                Ordering::Equal => {
                    v.push(*x1);
                    v.push(*x2);
                    p1.next();
                    p2.next();
                }
                Ordering::Greater => {
                    v.push(*x2);
                    p2.next();
                }
                Ordering::Less => {
                    v.push(*x1);
                    p1.next();
                }
            }
        } else {
            break;
        }
    }
    v.extend(p1);
    v.extend(p2);
    let l = v[p.len() - 1];
    let r = l + s;
    let cost1: i64 = p.iter().copied().take_while(|x| *x < l).map(|x| l - x).sum();
    let cost2: i64 = p.iter().copied().rev().take_while(|x| *x > r).map(|x| x - r).sum();
    (cost1 + cost2) as u64
}

fn main() {
    input! {
        n: usize,
        k: u64,
        xy: [(i64, i64); n],
    }

    let (mut x, mut y) = xy.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();
    x.sort_unstable();
    y.sort_unstable();
    
    let mut l = -1;
    let mut r = 10i64.pow(9);
    while r - l > 1 {
        let mid = (l + r) / 2;
        let cost = cost(&x, mid) + cost(&y, mid);
        if cost > k {
            l = mid;
        } else {
            r = mid;
        }
    }
    assert_eq!(l + 1, r);
    println!("{r}");
}
