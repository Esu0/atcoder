use std::collections::HashMap;

use proconio::{input, marker};

fn main() {
    input! {
        t: usize,
        cases: [marker::Bytes; t],
    }
    for s in cases {
        let n = s.len();
        let mut dp = vec![0u32; n + 1];
        let mut dp_min1 = HashMap::new();
        let mut dp_min2 = HashMap::new();
        dp_min1.insert(0i32, 0u32);
        let mut count = 0i32;
        for i in 0..n {
            dp_min1.entry(count).and_modify(|e| *e = (*e).min(dp[i])).or_insert(dp[i]);
            if s[i] == b'A' {
                count += 1;
                dp[i + 1] = dp_min2.get(&count).copied().unwrap_or(u32::MAX);
            } else {
                dp_min2.entry(count).and_modify(|e| *e = (*e).min(dp[i])).or_insert(dp[i]);
                count -= 2;
                dp[i + 1] = dp_min1.get(&count).copied().unwrap_or(u32::MAX);
            }
            dp[i + 1] = dp[i + 1].min(dp[i] + 1);
        }
        assert_eq!((n as u32 - dp[n]) % 3, 0);
        println!("{}", (n as u32 - dp[n]) / 3);
    }
}
