use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [f64; n],
    }

    let mut dp = vec![vec![]; n];
    dp[0] = vec![1. - p[0], p[0]];
    for i in 1..n {
        dp[i] = vec![dp[i - 1][0] * (1. - p[i])];
        for j in 1..=i {
            let tmp = dp[i - 1][j - 1] * p[i] + dp[i - 1][j] * (1. - p[i]);
            dp[i].push(tmp);
        }
        let tmp = dp[i - 1][i] * p[i];
        dp[i].push(tmp);
    }
    let ans = dp[n - 1].iter().skip(n / 2 + 1).sum::<f64>();
    println!("{}", ans);
}
