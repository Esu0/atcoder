use std::cmp::Reverse;

use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        t: i64,
        s: marker::Bytes,
        x: [i64; n],
    }
    let ant_neg = x.iter().enumerate().filter(|&(i, _)| s[i] == b'0').map(|(_, &xi)| xi).collect::<Vec<_>>();
    let mut ant_pos = x.iter().enumerate().filter(|&(i, _)| s[i] == b'1').map(|(_, &xi)| xi).collect::<Vec<_>>();
    ant_pos.sort_unstable_by_key(|&x| Reverse(x));
    eprintln!("{:?}", ant_neg);
    eprintln!("{:?}", ant_pos);
    let mut sum = 0;
    for &xi in &ant_neg {
        let p = ant_pos.partition_point(|&y| y >= xi);
        let p2 = ant_pos.partition_point(|&y| y >= xi - t * 2);
        eprintln!("{p} {p2}");
        sum += p2 - p;
    }
    println!("{sum}");
}
