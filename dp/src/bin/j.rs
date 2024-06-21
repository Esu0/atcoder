use proconio::input;

fn solve(dp: &mut [Vec<Vec<Option<f64>>>], i: usize, j: usize, k: usize, n: usize) -> f64 {
    if let Some(ans) = dp[i][j][k] {
        return ans;
    }
    if i + j + k == 0 {
        dp[i][j][k] = Some(0.);
        return 0.;
    }
    // let r = (n - (i + j + k)) as f64 / n as f64;
    dp[i][j][k] = Some(
        (if i > 0 {
            solve(dp, i - 1, j, k, n) * i as f64 / n as f64
        } else {
            0.
        } + if j > 0 {
            solve(dp, i + 1, j - 1, k, n) * j as f64 / n as f64
        } else {
            0.
        } + if k > 0 {
            solve(dp, i, j + 1, k - 1, n) * k as f64 / n as f64
        } else {
            0.
        } + 1.)
            / ((i + j + k) as f64 / n as f64),
    );

    dp[i][j][k].unwrap()
}

fn main() {
    input! {
        n: usize,
        a: [u8; n],
    }
    let mut count = [0usize; 3];
    for &ai in &a {
        count[ai as usize - 1] += 1;
    }
    let mut dp = vec![vec![vec![None; n + 1]; n + 1]; n + 1];
    let ans = solve(&mut dp, count[0], count[1], count[2], n);
    println!("{}", ans);
}
