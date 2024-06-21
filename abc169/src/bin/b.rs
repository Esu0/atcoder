use proconio::input;

fn main() {
    input! {
        a: [u64],
    }

    if a.iter().any(|&x| x == 0) {
        println!("0");
    } else {
        let mut ans = 1u64;
        for &x in a.iter() {
            ans = ans.saturating_mul(x);
            if ans > 1_000_000_000_000_000_000 {
                println!("-1");
                return;
            }
        }
        println!("{}", ans);
    }
}
