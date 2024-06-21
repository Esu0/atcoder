use proconio::input;


fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut dp = vec![vec![[0i64, 0]; n + 1]; n + 1];
    // for i in 0..n {
    //     dp[i][i] = [0, 0];
    // }
    for l in (0..n).rev() {
        for r in l + 1..=n {
            dp[l][r][0] = (dp[l + 1][r][1] + a[l]).max(dp[l][r - 1][1] + a[r - 1]);
            dp[l][r][1] = (dp[l + 1][r][0] - a[l]).min(dp[l][r - 1][0] - a[r - 1]);
        }
    }
    println!("{}", dp[0][n][0]);
}
