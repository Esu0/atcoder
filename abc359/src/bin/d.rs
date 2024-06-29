use proconio::{input, marker};

fn iter_bit(s: &[u8], f: &mut impl FnMut(usize), buf: &mut [u8]) {
    let k = s.len();
    if k == 0 {
        let mut idx = 0;
        let k = buf.len();
        let m = k / 2;
        if (0..m).all(|i| buf[i] == buf[k - 1 - i]) {
            return;
        }
        for (i, &bi) in buf.iter().enumerate() {
            idx |= (bi as usize) << i;
        }
        f(idx);
        return;
    }
    let (&first, s) = s.split_first().unwrap();
    match first {
        b'A' => {
            buf[k - 1] = 0;
            iter_bit(s, f, buf);
        }
        b'B' => {
            buf[k - 1] = 1;
            iter_bit(s, f, buf);
        }
        b'?' => {
            buf[k - 1] = 0;
            iter_bit(s, f, buf);
            buf[k - 1] = 1;
            iter_bit(s, f, buf);
        }
        _ => panic!(),
    }
}

fn is_palindrome(idx: usize, k: usize) -> bool {
    let m = k / 2;
    let a = idx & ((1 << m) - 1);
    let m2 = (k + 1) / 2;
    let b = (idx >> m2).reverse_bits() >> (usize::BITS - m as u32);
    a == b
}

const MOD: u64 = 998244353;
fn main() {
    input! {
        n: usize,
        k: usize,
        s: marker::Bytes,
    }
    let mut dp = vec![vec![0u64; 1 << (k - 1)]; n + 1];
    let mut buf = [0; 10];
    let buf = &mut buf[..k];
    let mask = (1 << (k - 1)) - 1;
    iter_bit(
        &s[..k],
        &mut |idx| {
            // eprintln!("{:01$b}", idx as u32, k);
            let idx = idx & mask;
            dp[k][idx] += 1
        },
        buf,
    );
    for i in k + 1..=n {
        if s[i - 1] != b'A' {
            for p in 0..1 << (k - 2) {
                let pa = p;
                let pb = p | (1 << (k - 2));
                let p = (p << 1) | 1;
                if !is_palindrome((pb << 1) | 1, k) {
                    dp[i][p] += dp[i - 1][pb];
                }
                dp[i][p] += dp[i - 1][pa];
                dp[i][p] %= MOD;
            }
        }
        if s[i - 1] != b'B' {
            for p in 0..1 << (k - 2) {
                let pa = p;
                let pb = p | (1 << (k - 2));
                let p = p << 1;
                if !is_palindrome(pa << 1, k) {
                    dp[i][p] += dp[i - 1][pa];
                }
                dp[i][p] += dp[i - 1][pb];
                dp[i][p] %= MOD;
            }
        }
    }
    // for (p, &dp) in dp[5].iter().enumerate() {
    //     eprintln!("{:02$b}: {}", p, dp, k - 1);
    // }
    println!("{}", dp[n].iter().sum::<u64>() % MOD);
}
