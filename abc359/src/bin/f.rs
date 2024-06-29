use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut queue = a.iter().enumerate().map(|(i, &ai)| (Reverse(ai * 3), i)).collect::<BinaryHeap<_>>();
    let mut d = vec![1u32; n];
    let mut ans = a.iter().sum::<u64>();
    for _ in 0..n - 2 {
        // eprintln!("{:?}", queue);
        let (Reverse(diff), i) = queue.pop().unwrap();
        d[i] += 1;
        queue.push((Reverse((2 * d[i] as u64 + 1) * a[i]), i));
        ans += diff;
    }
    println!("{}", ans);
}
