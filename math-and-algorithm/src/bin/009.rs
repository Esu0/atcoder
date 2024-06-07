use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }
    let mut dp = vec![vec![false; s + 1]; n];
    if let Some(t) = dp[0].get_mut(a[0]) {
        *t = true;
    }
    
    for i in 1..n {
        for j in 1..=s {
            // dp[i][j]: a[0..=i]までの整数をいくつか選んでj + 1が作れるか
            dp[i][j] = dp[i - 1][j] || (j >= a[i] && dp[i - 1][j - a[i]]);
        }
    }
    if dp[n - 1][s - 1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
