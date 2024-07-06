use std::collections::VecDeque;

use proconio::{input, marker};


fn main() {
    input! {
        n: usize,
        s: marker::Bytes,
        t: marker::Bytes,
    }
    let mut dp = vec![vec![u32::MAX; 1 << (n + 2)]; n + 1];
    let mut s_state = 0;
    for si in &s {
        s_state <<= 1;
        if *si == b'B' {
            s_state |= 1;
        }
    }
    s_state <<= 2;

    let mut t_state = 0;
    for ti in &t {
        t_state <<= 1;
        if *ti == b'B' {
            t_state |= 1;
        }
    }
    t_state <<= 2;

    dp[n][s_state] = 0;
    let mut queue = VecDeque::from([(n, s_state)]);
    while let Some((i, j)) = queue.pop_back() {
        let d = dp[i][j];
        // eprintln!("{i}, {j:00$b}: {d}", n + 2);
        if i == n && j == t_state {
            println!("{d}");
            return;
        }
        for k in 0..n + 1 {
            if (i.saturating_sub(1)..=i + 1).contains(&k) {
                continue;
            }
            let bik = n - k;
            let bii = n - i;
            let next_j = j & !(0b11 << bik) | (((j >> bik) & 0b11) << bii);
            // eprintln!("{i}, {j:00$b} -> {k}, {next_j:00$b}", n + 2);
            if dp[k][next_j] == u32::MAX {
                dp[k][next_j] = d + 1;
                queue.push_front((k, next_j));
            }
        }
    }
    println!("-1");

}
