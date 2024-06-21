use proconio::{input, marker};

fn main() {
    input! {
        s: marker::Bytes,
        t: marker::Bytes,
    }
    let mut dp = vec![vec![((usize::MAX, usize::MAX), 0); s.len() + 1]; t.len() + 1];
    for i in 1..=t.len() {
        for j in 1..=s.len() {
            if s[j - 1] == t[i - 1] {
                dp[i][j] = ((i - 1, j - 1), dp[i - 1][j - 1].1 + 1);
            }
            dp[i][j] = std::cmp::max_by_key(dp[i][j], dp[i - 1][j], |&(_, x)| x);
            dp[i][j] = std::cmp::max_by_key(dp[i][j], dp[i][j - 1], |&(_, x)| x);
        }
    }
    let mut ans = vec![];
    let mut i = t.len();
    let mut j = s.len();
    loop {
        let (next_i, next_j) = dp[i][j].0;
        if next_i > t.len() || next_j > s.len() {
            break;
        }
        assert_eq!(t[next_i], s[next_j]);
        ans.push(t[next_i]);
        i = next_i;
        j = next_j;
    }
    ans.reverse();
    println!("{}", std::str::from_utf8(&ans).unwrap());
}
