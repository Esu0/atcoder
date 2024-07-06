use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        w: [u64; n],
    }
    let mut boxes = vec![vec![]; n];
    for (&ai, &wi) in a.iter().zip(&w) {
        boxes[ai - 1].push(wi);
    }
    boxes.iter_mut().for_each(|b| b.sort_unstable_by_key(|&x| Reverse(x)));
    let ans = boxes.iter().flat_map(|b| b.iter().skip(1)).sum::<u64>();
    println!("{}", ans);
}
