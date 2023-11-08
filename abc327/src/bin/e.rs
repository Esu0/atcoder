use std::vec;

use proconio::input;

fn rate(sum: f64, k: usize) -> f64 {
    sum / ((1.0 - (0.9f64).powi(k as i32)) / (1. - 0.9)) - 1200. / (k as f64).sqrt()
}

fn main() {
    input! {
        n: usize,
        mut p: [u32; n],
    }

    p.reverse();

    // dp[i][j] p[0..=i]の範囲でj個選んだ時の評価値
    let mut dp = vec![vec![0.; n + 1]; n];
    // dp[0] = p.iter().map(|pn| *pn as f64).collect();
    dp[0][1] = p[0] as f64;
    for i in 1..n {
        for j in 1..=(i + 1) {
            dp[i][j] = std::cmp::max_by(
                dp[i - 1][j - 1] + (0.9f64).powi(j as i32 - 1) * p[i] as f64,
                dp[i - 1][j],
                |x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal),
            );
        }
    }
    let ans = dp[n - 1]
        .iter()
        .enumerate()
        .map(|(j, sum)| rate(*sum, j))
        .max_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap();

    // println!("{dp:?}");
    println!("{ans}");
}
