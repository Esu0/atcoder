use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [(usize, usize); n],
    }
    let mut dp = vec![vec![usize::MAX; 1000 * n + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..=n {
        dp[i][0] = 0;
        for j in 1..1000 * n + 1 {
            if j >= wv[i - 1].1 {
                dp[i][j] = dp[i - 1][j].min(dp[i - 1][j - wv[i - 1].1].saturating_add(wv[i - 1].0));
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }
    // eprintln!("{:?}", dp[n].iter().enumerate().filter(|&(_, &wi)| wi <= w).collect::<Vec<_>>());
    let ans = dp[n].iter().rposition(|&x| x <= w).unwrap();
    println!("{}", ans);
}
