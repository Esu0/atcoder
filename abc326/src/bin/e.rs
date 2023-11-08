use proconio::input;

fn mpow(mut x: u64, mut exp: u64, m: u64) -> u64 {
    let mut ans = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            ans = ans * x % m;
        }
        x = x * x % m;
        exp >>= 1;
    }
    ans
}

fn minv(x: u64, m: u64) -> u64 {
    mpow(x, m - 2, m)
}

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    const MOD: u64 = 998244353;
    let mut dp = vec![0; n + 1];
    let mut sum = 0;
    let n_inv = minv(n as u64, MOD);
    for i in (0..n).rev() {
        sum = (sum + dp[i + 1] + a[i]) % MOD;
        dp[i] = sum * n_inv % MOD;
    }

    print!("{}", dp[0]);
}
