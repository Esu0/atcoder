use std::vec;

use proconio::{input, marker};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [marker::Bytes; h],
    }
    const MODULO: u64 = 1000000007;
    let mut dp = vec![vec![0u64; w]; h];
    dp[0][0] = 1;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == b'#' {
                continue;
            }
            dp[i][j] += dp[i].get(j.wrapping_sub(1)).copied().unwrap_or_default()
                + dp.get(i.wrapping_sub(1)).map_or(0, |v| v[j]);
            dp[i][j] %= MODULO;
        }
    }

    println!("{}", dp[h - 1][w - 1]);
}
