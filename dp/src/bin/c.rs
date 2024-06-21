use proconio::input;

fn main() {
    input! {
        n: usize,
        abc: [(u32, u32, u32); n],
    }
    let mut dp = vec![[0; 3]; n];
    dp[0] = [abc[0].0, abc[0].1, abc[0].2];
    for i in 1..n {
        dp[i][0] = abc[i].0 + dp[i - 1][1].max(dp[i - 1][2]);
        dp[i][1] = abc[i].1 + dp[i - 1][0].max(dp[i - 1][2]);
        dp[i][2] = abc[i].2 + dp[i - 1][0].max(dp[i - 1][1]);
    }
    println!("{}", dp[n - 1].iter().max().unwrap());
}
