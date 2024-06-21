use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut dp = vec![false; k + 1];
    for i in 1..=k {
        dp[i] = a.iter().any(|&ai| i >= ai && !dp[i - ai]);
    }
    if dp[k] {
        println!("First");
    } else {
        println!("Second");
    }
}
