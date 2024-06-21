use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut count = [0u32; 3];
    for &ai in &a {
        count[(ai - 1) as usize] += 1;
    }
    let mut ans = 0;
    for &c in &count {
        if c == 0 {
            continue;
        }
        ans += (c as u64 * (c as u64 - 1)) / 2;
    }
    println!("{}", ans);
}
